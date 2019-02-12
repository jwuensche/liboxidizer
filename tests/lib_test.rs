use websocket;
use std::{time,thread};

mod common;

#[test]
fn test_connection() {
    common::main();

    //just a short period to wait for thread without communication
    let seconds = time::Duration::from_secs(1);
    thread::sleep(seconds);

    liboxidizer::connect("liboxidizer_integration", "ws://127.0.0.1:50000").unwrap();

}
