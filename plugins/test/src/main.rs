extern crate capnp;
extern crate capnp_rpc;
extern crate tokio;
extern crate futures;
extern crate rand;

use futures::prelude::*;
use tokio::prelude::*;

use futures::future::{loop_fn, Loop};

use protocol_capnp::rpc;
use capnp::capability::{Request, Response};
use capnp_rpc::{twoparty, rpc_twoparty_capnp, RpcSystem};

use tokio::executor::current_thread;
use tokio::runtime::current_thread::Runtime;

use std::time::{Instant, Duration};

mod protocol_capnp {
    include!(concat!(env!("OUT_DIR"), "/protocol_capnp.rs"));
}

fn main() {
    let host_addr = std::env::args()
        .nth(1).expect("Missing host address")
        .parse().expect("Invalid host address");

    let batch_size = std::env::args()
        .nth(2).expect("Missing batch size")
        .parse().expect("Batch size must be a number");

    let mut runtime = Runtime::new()
        .expect("Failed to initialize tokio runtime");

    let work = tokio::net::TcpStream::connect(&host_addr)
        .map_err(RpcError::ConnectError)
        .and_then(|sock| {
            let rpc_client = match sock.split() {
                (reader, writer) => bootstrap_rpc_client(reader, writer)
            };

            loop_fn(State::default(), move |state| {
                println!("{}", state);

                send_batch(&rpc_client, batch_size)
                    .map(move |duration| state.add(batch_size, duration))
                    .map(|new_state| Loop::Continue::<(), _>(new_state))
            })
        });

    runtime.block_on(work)
        .expect("failed to run work");
}

#[derive(Default)]
struct State {
    duration: Duration,
    requests_sent: usize
}

impl State {
    fn add(&self, amt: usize, duration: Duration) -> Self {
        Self {
            duration: self.duration + duration,
            requests_sent: self.requests_sent + amt
        }
    }
}

type AddRequest = Request<rpc::add_params::Owned, rpc::add_results::Owned>;
type AddResponse = Response<rpc::add_results::Owned>;

fn generate_add_request(client: &rpc::Client, a: i32, b: i32)
    -> AddRequest
{
    let mut request = client.add_request();

    match request.get() {
        mut params => {
            params.set_a(a);
            params.set_b(b);
        }
    };

    request
}

fn bootstrap_rpc_client<R, W>(reader: R, writer: W) -> rpc::Client
where
    R: Read + 'static,
    W: Write + 'static
{
    let network = twoparty::VatNetwork::new(
        reader, writer,
        rpc_twoparty_capnp::Side::Client,
        Default::default()
    );

    let mut rpc_system = RpcSystem::new(Box::new(network), None);

    let client = rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);

    current_thread::spawn(rpc_system
        .map_err(|e| panic!("Failed to spawn rpc system: {}", e))
    );

    client
}

fn send_add_request(client: &rpc::Client)
    -> impl Future<Item = (), Error = RpcError>
{
    use rand::random;

    let (a, b) = (random(), random());
    let request = generate_add_request(client, a, b);

    let check_response = move |response: AddResponse| {
        let expected_result = a as i64 + b as i64;

        let result = response.get()
            .map_err(RpcError::InvalidResponse)?
            .get_result();

        match result == expected_result {
            true  => Ok(()),
            false => Err(RpcError::LogicError)
        }
    };

    request.send().promise
        .map_err(RpcError::SendError)
        .and_then(check_response)
}

fn send_batch(rpc_client: &rpc::Client, amt: usize)
    -> impl Future<Item = Duration, Error = RpcError>
{
    let requests = (0_usize..amt)
        .map(|_| send_add_request(&rpc_client))
        .collect::<Vec<_>>();

    let send_start = Instant::now();

    futures::future::join_all(requests)
        .map(move |_| Instant::now() - send_start)
}

#[derive(Debug)]
enum RpcError {
    ConnectError(std::io::Error),
    SendError(capnp::Error),
    InvalidResponse(capnp::Error),
    LogicError,
}

use std::fmt;

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let total_secs = self.duration.as_secs() as f64 +
            (self.duration.subsec_nanos() as f64 / 1_000_000_000_f64);

        let rps = match self.requests_sent {
            0 => 0_f64,
            requests_sent => requests_sent as f64 / total_secs
        };

        write!(f,
            "REQS: {} | TIME: {:.5}s | RPS: {:.5}",
            self.requests_sent, total_secs, rps
        )
    }
}
