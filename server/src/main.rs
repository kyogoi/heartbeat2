mod auth;
mod config;
mod db;
mod web;

use config::Config;
use db::Database;
use rumqttd::{Broker, Config, Notification};
use std::{thread, time::Duration};

fn main() {
    let config = config::Config::builder()
        .add_source(config::File::with_name("rumqttd.toml"))
        .build()
        .expect("Failed to build config")
        .try_deserialize()
        .expect("Failed to deserialize config");

    let mut broker = Broker::new(config);
    let alerts = broker.alerts().expect("Failed to receive alerts");
    let (mut link_tx, mut link_rx) = broker.link("yea").expect("Failed to link broker");

    link_tx
        .subscribe("hello/+/world")
        .expect("Failed to subscribe to topic");

    link_tx
        .subscribe("test")
        .expect("Failed to subscribe to topic 'test'");

    thread::spawn(move || {
        loop {
            let notification = match link_rx.recv().unwrap() {
                Some(v) => v,
                None => {
                    continue;
                }
            };

            match notification {
                Notification::Forward(forward) => {
                    println!(
                        "✓ RECEIVED MESSAGE! Topic = {:?}, Payload = {} bytes",
                        forward.publish.topic,
                        forward.publish.payload.len()
                    );
                }
                v => {}
            }
        }
    });

    let handle = thread::spawn(move || {
        loop {
            if let Ok(alert) = alerts.recv() {}
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        broker.start().expect("Failed to start broker");
    });

    println!("\n=== Server is now running and listening ===\n");
    handle.join().unwrap();
}
