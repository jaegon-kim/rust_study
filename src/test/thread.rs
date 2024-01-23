#![allow(unused)]

use std::thread;
use std::time::Duration;


fn wait_test(msg: &str) {
    for i in 1..10 {
        println!("{} {}", msg, i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn server() {
    wait_test("server");
}

fn client() {
    wait_test("client");
}

pub fn test_thread() {
    println!("test_thread");
    let server_handle = thread::spawn(
        || {
            server();
        }
    );
    let client_handle = thread::spawn(
        || {
            client();
        }
    );
    client_handle.join().unwrap();
    server_handle.join().unwrap();
}

