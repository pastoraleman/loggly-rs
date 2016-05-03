extern crate loggly;

use loggly::{LogglyClient, LogglyMessage};

fn main() {

    let mut msg = LogglyMessage::new();

    msg.from = "Loggly Client for Rustlang".to_string();
    msg.message = "This is a test message from the world of Rust!".to_string();
    msg.tags.push("rust".to_string());
    msg.tags.push("hi!".to_string());

    let client = LogglyClient::new("your-api-key-here!".to_string());

    match client.send(msg) {
        Ok(response) => println!("{}", response),
        Err(err) => panic!(err)
    }


}
