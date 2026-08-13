//! Simple usage example for `subsecond_async`, using macros.
//!
//! Toy around with `dx serve --hotpatch --package subsecond-async --example macros`.

use std::time::Duration;

use tokio::time::{interval, sleep};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    dioxus_devtools::connect_subsecond();

    let mut tick = interval(Duration::from_secs(1));
    loop {
        tick.tick().await;
        on_tick_async().await;
        on_tick_sync();
    }
}

#[subsecond_async_macros::subsecond]
async fn on_tick_async() {
    for i in 0..5 {
        println!("Hello async, {i}");
        sleep(Duration::from_millis(10)).await;
    }
}

#[subsecond_async_macros::subsecond]
fn on_tick_sync() {
    for i in 0..5 {
        println!("Hello sync, {i}");
    }
}
