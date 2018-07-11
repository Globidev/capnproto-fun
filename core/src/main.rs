#[macro_use]
extern crate serde_derive;

extern crate futures;
extern crate tokio;

use futures::prelude::*;

mod discord;

pub fn main() {
    let work = discord::gateway::connect()
        .map_err(|e| println!("{:?}", e))
        .and_then(|(events, _sink)| {
            events.for_each(|e| {
                println!("{:?}", e);
                Ok(())
            }).map_err(|e| println!("{:?}", e))
        });

    tokio::run(work);
}

