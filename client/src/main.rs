use futures::executor::block_on;
use paho_mqtt as mqtt;
use std::{env, process};

fn main() {
    // Command-line option(s)
    let host = env::args()
        .nth(1)
        .unwrap_or_else(|| "localhost:1883".to_string());

    println!("Connecting to the MQTT server at '{}'", host);

    // Create the client
    let cli = mqtt::AsyncClient::new(host).unwrap_or_else(|err| {
        eprintln!("Error creating the client: {}", err);
        process::exit(1);
    });

    println!("Client created successfully!");

    if let Err(err) = block_on(async {
        // Connect with default options and wait for it to complete or fail
        // The default is an MQTT v3.x connection.
        println!("Attempting to connect...");
        cli.connect(None).await?;
        println!("✓ Connected successfully!");

        // Create a message and publish it
        println!("Publishing a message on the topic 'test'");
        let msg = mqtt::Message::new("test", "Hello Rust MQTT world!", mqtt::QOS_1);
        cli.publish(msg).await?;
        println!("✓ Message published!");

        // Give the server a moment to process the message
        println!("Waiting a moment before disconnecting...");
        std::thread::sleep(std::time::Duration::from_millis(500));

        // Disconnect from the broker
        println!("Disconnecting");
        cli.disconnect(None).await?;
        println!("✓ Disconnected successfully!");

        Ok::<(), mqtt::Error>(())
    }) {
        eprintln!("ERROR: {}", err);
    }

    println!("\nClient finished!");
}
