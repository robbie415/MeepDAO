use std::{thread::sleep, time::Duration};

#[allow(dead_code)]
pub mod rpc_client;

pub fn delay() {
    sleep(Duration::new(0, 500_000_000));
}
