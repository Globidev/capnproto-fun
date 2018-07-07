extern crate tokio_tungstenite;
extern crate tungstenite;
extern crate futures;
extern crate url;
extern crate tokio;
extern crate tokio_tls;

use self::futures::prelude::*;
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

pub mod event;

use self::event::{Event, MessageIn, MessageOut};

// const BASE_URL: &'static str = "https://discordapp.com/api";

type WsStream = WebSocketStream<StreamSwitcher<TcpStream, TlsStream<TcpStream>>>;

pub type EventStream = mpsc::Receiver<Event>;
type EventSink = mpsc::Sender<Event>;

struct Gateway {
    stream: SplitStream<WsStream>,
    sink: SplitSink<WsStream>,
    event_sink: EventSink,
    action_sink: InternalActionSink,
}

struct InternalActionSink {
    tx: mpsc::Sender<MessageOut>
}

pub struct ActionSink {
    tx: mpsc::Sender<MessageOut>
}

// impl WsSink {
//     fn new(sink: SplitSink<WsStream>) -> Self {
//         let (tx, rx) = mpsc::channel(0);
//         self::tokio::executor::spawn(send_messages(sink, rx));

//         Self { tx }
//     }

//     pub fn identify(&mut self) {
//         let token = String::from("NDY0MjM5NzA5NzQ1NTc3OTg1.Dh8EwA.viWCf4SN6rDNrzAP1ONF50NQCmw");
//         let properties = event::op::IdentifyProperties {
//             os: String::from("linux"),
//             browser: String::from("none"),
//             device: String::from("globiworkstation")
//         };

//         let identify_data = event::op::Identify { token, properties, ..Default::default() };
//         let identify_event = event::MessageOut::Identify(identify_data);

//         self.tx.try_send(identify_event)
//             .expect("error queuing identify")
//     }

//     pub fn heartbeat(&mut self, sequence_number: Option<u32>) {
//         let heartbeat_event = MessageOut::Heartbeat(sequence_number);

//         self.tx.try_send(heartbeat_event)
//             .expect("error queuing heartbeat")
//     }
// }

fn send_messages(sink: SplitSink<WsStream>, rx: mpsc::Receiver<MessageOut>)
    -> impl Future<Item = (), Error = ()>
{
    rx.fold(sink, |sink, event| {
        let payload = event::to_raw_payload(event).unwrap();
        println!("OUT: {:?}", payload);
        sink.send(WsMessage::Text(payload))
            .map_err(|e| println!("{:?}", e))
    }).map(|_sink| ())
}

fn run_gateway(gateway: Gateway) -> impl Future<Item = (), Error = ()> {

}

pub fn connect() -> impl Future<Item = (EventStream, ActionSink), Error = WsError> {
    use self::tungstenite::handshake::client::Request;
    use self::url::Url;

    const WS_URL: &'static str = "wss://gateway.discord.gg/?v=6&encoding=json";

    let url = Url::parse(WS_URL).expect("Invalid Gateway URL");
    // let req:  = url.into();
    self::tokio_tungstenite::connect_async(url.into())
        .map(|(ws_stream, _response)| {
            let (sink, stream) = ws_stream.split();
            let (event_sink, event_stream) = mpsc::channel(0);
            let gateway = Gateway { stream, sink, event_sink };

            self::tokio::spawn(run_gateway(gateway));

            let events = stream
                .filter_map(text_data) // Keep only text messages
                .filter_map(|raw_data| {
                    event::from_raw_payload(&raw_data)
                        .map_err(|err| println!("{:?}", err))
                        .ok()
                });

            (event_stream, WsSink::new(sink))
        })
}

fn text_data(message: WsMessage) -> Option<String> {
    match message {
        WsMessage::Text(data) => Some(data),
        _                     => None
    }
}
