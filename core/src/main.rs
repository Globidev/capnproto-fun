#[macro_use]
extern crate serde_derive;

extern crate futures;
extern crate tokio;

use futures::prelude::*;

mod gateway;

use std::time::{Instant, Duration};

use std::sync::{Arc, RwLock};

pub fn main() {
    // let sequence_number = Arc::new(RwLock::new(None));

    // let work = gateway::connect()
    //     .map_err(|e| println!("{:?}", e))
    //     .and_then(move |(events, mut sink)| {
    //         events.for_each(move |e| {
    //             println!("IN: {:?}", e);

    //             match e {
    //                 gateway::event::EventIn::Dispatch(d) => {
    //                     *sequence_number.write().unwrap() = Some(d.sequence_number);
    //                 },

    //                 gateway::event::EventIn::Hello(h) => {
    //                     sink.identify();

    //                     let mut hb_sink = sink.clone();
    //                     let n = Arc::clone(&sequence_number);
    //                     let send_heartbeats = tokio::timer::Interval::new(
    //                         Instant::now() + Duration::from_millis(h.heartbeat_interval), Duration::from_millis(h.heartbeat_interval)
    //                     )
    //                     .for_each(move |_| {
    //                         hb_sink.heartbeat(*n.read().unwrap());
    //                         Ok(())
    //                     })
    //                     .map_err(|e| println!("{}", e));

    //                     tokio::spawn(send_heartbeats);
    //                 },

    //                 _ => {}
    //             }

    //             Ok(())
    //         }).map_err(|e| println!("{:?}", e))
    //     });

    let work = gateway::connect()
        .map_err(|e| println!("{:?}", e))
        .and_then(|(events, mut sink)| {
            events.for_each(|e| {
                println!("{:?}", e);
                Ok(())
            }).map_err(|e| println!("{}", e))
        });

    tokio::run(work);
}

