#![crate_name = "zeromq_test"]

//! Hello World server in Rust
//! Binds REP socket to tcp://*:5555
//! Expects "Hello" from client, replies with "World"

use std::thread;
use std::time::Duration;


fn main() {
    println!("Hello, world! rust zmq test");
    let context = zmq::Context::new();
    let responder = context.socket(zmq::REP).unwrap();

    assert!(responder.bind("tcp://*:{5556}").is_ok());
    if responder.bind("tcp://*:5556").is_err() {
        println!("bind error");
        // print what error is:
        match responder.bind("tcp://*:5556") {
            Ok(_) => println!("bind ok"),
            Err(e) => println!("bind error: {}", e),
        }
        // terminate program
        return;
    }
    else {
        println!("bind ok");
    }
    let mut msg = zmq::Message::new();
    loop {
        responder.recv(&mut msg, 0).unwrap();
        println!("Received {}", msg.as_str().unwrap());
        thread::sleep(Duration::from_millis(1000));
        responder.send("World", 0).unwrap();
    }
}