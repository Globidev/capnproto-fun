#[macro_use]
extern crate serde_derive;

extern crate futures;
extern crate tokio;

use futures::prelude::*;

mod gateway;

pub fn main() {
    let work = gateway::connect()
        .map_err(|e| println!("{:?}", e))
        .and_then(|(events, mut sink)| {
            events.for_each(|e| {
                println!("{:?}", e);
                Ok(())
            }).map_err(|e| println!("{:?}", e))
        });

    tokio::run(work);
}

