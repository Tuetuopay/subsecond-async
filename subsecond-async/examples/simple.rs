//! Simple usage example for `subsecond_async`.
//!
//! Toy around with `dx serve --hotpatch --package subsecond-async --example simple`.

use std::time::Duration;

use tokio::time::{interval, sleep};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    dioxus_devtools::connect_subsecond();

    let mut tick = interval(Duration::from_secs(1));
    loop {
        tick.tick().await;
        subsecond_async::call(|| on_tick()).await;
    }
}

async fn on_tick() {
    for i in 0..5 {
        println!("Hello, {i}");
        sleep(Duration::from_millis(10)).await;
    }
}
