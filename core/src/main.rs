extern crate capnp;
extern crate capnp_rpc;
extern crate futures;
extern crate tokio;

pub mod protocol_capnp {
    include!(concat!(env!("OUT_DIR"), "/protocol_capnp.rs"));
}

use protocol_capnp::{rpc, unit};
use capnp::serialize_packed;
use capnp::capability::Promise;

use futures::prelude::*;
use tokio::prelude::*;

use std::time::{Instant, Duration};

struct ServerState {
}

impl rpc::Server for ServerState {
    fn subscribe(&mut self, params: rpc::SubscribeParams, mut results: rpc::SubscribeResults)
        -> Promise<(), ::capnp::Error>
    {
        results.get().init_result()
            .init_ok();
        // results.get().init_result()
        //     .init_err()
        //     .set_reason("Nope, loser");
        Promise::ok(())
    }

    fn add(&mut self, params: rpc::AddParams, mut results: rpc::AddResults)
        -> Promise<(), ::capnp::Error>
    {
        params.get()
            .map(|ps| {
                let result = {
                    let (a, b) = (ps.get_a() as i64, ps.get_b() as i64);
                    a + b
                };
                results.get().set_result(result);
                Promise::ok(())
            })
            .unwrap_or_else(|e| Promise::err(e))
    }
}

// fn write_hello(writer: &mut impl Write) -> ::std::io::Result<()> {
//     let mut message = ::capnp::message::Builder::new_default();

//     {
//         let event = message.init_root::<event::Builder>();
//         let mut m = event.init_new_message();
//         m.set_text("Hello");
//     }

//     serialize_packed::write_message(writer, &message)
// }

// fn write_ping(writer: &mut impl Write) -> ::std::io::Result<()> {
//     let mut message = ::capnp::message::Builder::new_default();

//     {
//         let mut event = message.init_root::<event::Builder>();
//         event.set_ping(());
//     }

//     serialize_packed::write_message(writer, &message)
// }

// fn read_e(reader: &mut impl BufRead) -> ::capnp::Result<()> {
//     let message_reader = serialize_packed::read_message(
//         reader,
//         ::capnp::message::ReaderOptions::new()
//     )?;

//     let event = message_reader.get_root::<event::Reader>()?;

//     match event.which()? {
//         event::NewMessage(m) => println!("Message: {}", m?.get_text()?),
//         event::Ping(())      => println!("PING")
//     }

//     Ok(())
// }

pub fn main() {
    use std::net::SocketAddr;
    use capnp_rpc::{twoparty, rpc_twoparty_capnp, RpcSystem};
    use std::str::FromStr;
    use tokio::executor::current_thread;

    let mut rt = tokio::runtime::current_thread::Runtime::new()
        .expect("rt");

    let listener = ::tokio::net::TcpListener::bind(
        &SocketAddr::from_str("127.0.0.1:7788").unwrap()
    ).expect("Failed to bind listener");

    let work = listener.incoming().for_each(move |stream| {
        let (reader, writer) = stream.split();

        let network = twoparty::VatNetwork::new(
            reader, writer,
            rpc_twoparty_capnp::Side::Server,
            Default::default()
        );

        let proxy = rpc::ToClient::new(
            ServerState{ }
        ).from_server::<::capnp_rpc::Server>();

        let rpc_system = RpcSystem::new(
            Box::new(network),
            Some(proxy.client)
        );

        current_thread::spawn(rpc_system.map_err(|e| println!("{}", e)));

        Ok(())
    });

    rt.spawn(work.map_err(|e| println!("{}", e)));
    rt.run().expect("failed to run work");
}
