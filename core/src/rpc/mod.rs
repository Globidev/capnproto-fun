extern crate capnp;
extern crate capnp_rpc;
extern crate futures;
extern crate tokio;

use tokio::executor::current_thread;
use tokio::runtime::current_thread::Runtime;

mod protocol_capnp {
    include!(concat!(env!("OUT_DIR"), "/protocol_capnp.rs"));
}

fn run_rpc_server() {
    let host_addr = std::env::args()
        .nth(1).expect("Missing host address")
        .parse().expect("Invalid host address");

    let mut runtime = Runtime::new()
        .expect("Failed to initialize tokio runtime");

    let listener = tokio::net::TcpListener::bind(&host_addr)
        .expect("Failed to bind listener");

    let work = listener.incoming()
        .for_each(move |stream| {
            let rpc_system = match stream.split() {
                (reader, writer) => make_rpc_system(reader, writer)
            };

            current_thread::spawn(rpc_system
                .map_err(|e| panic!("Failed to spawn rpc system: {}", e))
            );

            Ok(())
        });

    runtime.block_on(work)
        .expect("failed to run work");
}

use capnp::capability::Promise;
use capnp_rpc::{twoparty, rpc_twoparty_capnp, RpcSystem};

use protocol_capnp::rpc;

struct ServerState { }

fn make_rpc_system<R, W>(reader: R, writer: W)
    -> RpcSystem<rpc_twoparty_capnp::Side>
where
    R: Read + 'static,
    W: Write + 'static
{
    let network = twoparty::VatNetwork::new(
        reader, writer,
        rpc_twoparty_capnp::Side::Server,
        Default::default()
    );

    let proxy = rpc::ToClient::new(ServerState { }).
        from_server::<capnp_rpc::Server>();

    RpcSystem::new(Box::new(network), Some(proxy.client))
}

impl rpc::Server for ServerState {
    fn add(&mut self, params: rpc::AddParams, mut results: rpc::AddResults)
        -> Promise<(), ::capnp::Error>
    {
        params.get()
            .map(|add_params| {
                let result = {
                    let a = add_params.get_a() as i64;
                    let b = add_params.get_b() as i64;
                    a + b
                };
                results.get().set_result(result);
                Promise::ok(())
            })
            .unwrap_or_else(|e| Promise::err(e))
    }
}
