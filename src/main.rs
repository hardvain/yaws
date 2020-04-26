use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write, BufReader};
use std::env;
use std::str;
use std::collections::HashMap;
use std::io::*;

use crate::http::*;

mod stream;
mod http;

fn main() {
    let args: Vec<String> = env::args().collect();
    let port = &args[1];
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();
    println!("Server listening on port {}", port);

    for stream in listener.incoming() {
        stream::handle_request(stream)
    }
    // close the socket server
    drop(listener);
}
