extern crate tokio_tungstenite;
extern crate tungstenite;
extern crate futures;
extern crate url;
extern crate tokio;
extern crate tokio_tls;

use self::futures::prelude::*;
use self::futures::future;
use self::futures::stream::{SplitStream, SplitSink};
use self::futures::sync::mpsc;

use self::tokio_tungstenite::{
    WebSocketStream,
    stream::Stream as StreamSwitcher
};

use self::tokio::net::TcpStream;
use self::tokio_tls::TlsStream;

use self::tungstenite::{
    Message as WsMessage,
    Error as WsError
};

use std::sync::{Arc, RwLock};

use std::time::{Duration, Instant};

pub mod event;

use self::event::{Event, MessageIn, MessageOut};

// const BASE_URL: &'static str = "https://discordapp.com/api";

type WsStream = WebSocketStream<StreamSwitcher<TcpStream, TlsStream<TcpStream>>>;

#[allow(dead_code)] // BUG: https://github.com/rust-lang/rust/issues/18290
type EventStream = mpsc::Receiver<Event>;

type EventSink = mpsc::Sender<Event>;

type BoxFuture<T, E> = Box<Future<Item = T, Error = E> + Send + 'static>;

struct Gateway {
    stream: SplitStream<WsStream>,
    sink: SplitSink<WsStream>,
    event_sink: EventSink,
    action_stream: mpsc::Receiver<MessageOut>,
    action_sink: InternalActionSink,
}

struct InternalActionSink {
    tx: mpsc::Sender<MessageOut>,
    sequence_number: Arc<RwLock<Option<u32>>>,
}

impl InternalActionSink {
    fn send_heartbeat(self, sequence_number: u32)
        -> impl Future<Item = Self, Error = ()>
    {
        let heartbeat_message = MessageOut::Heartbeat(Some(sequence_number));

        self.send(heartbeat_message)
    }

    fn send_heartbeats(&self, interval: Duration)
        -> impl Future<Item = (), Error = ()>
    {
        let heartbeats_tx = self.tx.clone();
        let atomic_sequence_number = Arc::clone(&self.sequence_number);

        self::tokio::timer::Interval::new(Instant::now() + interval, interval)
            .map_err(|e| println!("Heartbeat timer error: {}", e))
            .fold(heartbeats_tx, move |tx, _| {
                let sequence_number = atomic_sequence_number.read().unwrap();
                let heartbeat_message = MessageOut::Heartbeat(*sequence_number);
                tx.send(heartbeat_message)
                    .map_err(|e| println!("Failed to queue heartbeat: {}", e))
            })
            .map(|_tx| ())
    }

    fn identify(self) -> impl Future<Item = Self, Error = ()> {
        let token = String::from("NDY0MjM5NzA5NzQ1NTc3OTg1.Dh8EwA.viWCf4SN6rDNrzAP1ONF50NQCmw");
        let properties = event::op::IdentifyProperties {
            os: String::from("linux"),
            browser: String::from("none"),
            device: String::from("globiworkstation")
        };

        let identify_data = event::op::Identify { token, properties, ..Default::default() };
        let identify_message = MessageOut::Identify(identify_data);

        self.send(identify_message)
    }

    fn update_sequence_number(&mut self, new_sequence_number: u32) {
        *self.sequence_number.write().unwrap() = Some(new_sequence_number);
    }

    fn send(self, message: MessageOut) -> impl Future<Item = Self, Error = ()> {
        let Self { tx, sequence_number } = self;

        tx.send(message)
            .map_err(|e| println!("Failed to queue Message: {}", e))
            .map(|tx| Self { tx, sequence_number })
    }
}

pub struct ActionSink {
    tx: mpsc::Sender<MessageOut>
}

struct GatewayState {
    action_sink: InternalActionSink,
    event_sink: EventSink,
}

fn process_message(state: GatewayState, message: MessageIn)
    -> impl Future<Item = GatewayState, Error = ()>
{
    use self::MessageIn::*;

    match message {
        Dispatch(seq, event) => match state {
            GatewayState { mut action_sink, event_sink } => {
                action_sink.update_sequence_number(seq);
                let state_after_queue = event_sink.send(event)
                    .map(|event_sink| GatewayState {
                        action_sink,
                        event_sink
                    })
                    .map_err(|e| println!("Error queuing event: {}", e));
                Box::new(state_after_queue) as BoxFuture<_, _>
            }
        },
        HeartBeat(seq) => match state {
            GatewayState { action_sink, event_sink } => {
                let state_after_queue = action_sink.send_heartbeat(seq)
                    .map(|action_sink| GatewayState {
                        action_sink,
                        event_sink
                    });
                Box::new(state_after_queue)
            }
        },
        Reconnect => {
            Box::new(future::ok(state))
        },
        InvalidSession => {
            Box::new(future::ok(state))
        },
        Hello(hello) => match state {
            GatewayState { action_sink, event_sink } => {
                let interval = Duration::from_millis(hello.heartbeat_interval);
                let heartbeats = action_sink.send_heartbeats(interval);
                self::tokio::spawn(heartbeats);

                let state_after_queue = action_sink.identify()
                    .map(|action_sink| GatewayState {
                        action_sink,
                        event_sink
                    });

                Box::new(state_after_queue)
            }
        },
        HeartbeatAck => {
            Box::new(future::ok(state))
        },
    }
}

fn run_gateway(gateway: Gateway) -> impl Future<Item = (), Error = ()> {
    let state = GatewayState {
        action_sink: gateway.action_sink,
        event_sink: gateway.event_sink
    };

    let process_messages = gateway.stream
        .map_err(|e| println!("Event input error: {:?}", e))
        .filter_map(text_data)
        .fold(state, move |state, payload| {
            println!("<< {:?}", payload);

            let message = event::from_raw_payload(&payload)
                .expect("invalid input payload");

            process_message(state, message)
        })
        .map(|_| ());

    let process_actions = gateway.action_stream
        .fold(gateway.sink, |sink, message| {
            println!(">> {:?}", message);

            let payload = event::to_raw_payload(message)
                .expect("invalid output payload");

            sink.send(WsMessage::Text(payload))
                .map_err(|e| println!("Failed to send message {:?}", e))
        })
        .map(|_sink| ());

    process_messages
        .join(process_actions)
        .map(|_| ())
}

pub fn connect()
    -> impl Future<Item = (EventStream, ActionSink), Error = WsError>
{
    use self::tungstenite::handshake::client::Request;
    use self::url::Url;

    const WS_URL: &'static str = "wss://gateway.discord.gg/?v=6&encoding=json";

    let url = Url::parse(WS_URL)
        .expect("Invalid Gateway URL");

    self::tokio_tungstenite::connect_async(Request::from(url))
        .map(|(ws_stream, _response)| {
            let (sink, stream) = ws_stream.split();

            let (event_tx, event_rx) = mpsc::channel(0);
            let (action_tx, action_rx) = mpsc::channel(0);

            let gateway = Gateway {
                stream, sink,
                event_sink: event_tx,
                action_stream: action_rx,
                action_sink: InternalActionSink {
                    tx: action_tx.clone(),
                    sequence_number: Default::default()
                },
            };

            self::tokio::spawn(run_gateway(gateway));

            (event_rx, ActionSink { tx: action_tx })
        })
}

fn text_data(message: WsMessage) -> Option<String> {
    match message {
        WsMessage::Text(data) => Some(data),
        _                     => None
    }
}
