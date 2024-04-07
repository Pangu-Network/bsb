// main.rs
use crate::bridge::Bridge;

mod bridge;
mod message_protocol;
mod utils;

fn main() {
    let bridge = Bridge::new();
    if bridge.start_bridge() {
        println!("Bridge started successfully.");
    } else {
        println!("Failed to start the bridge.");
    }
}
