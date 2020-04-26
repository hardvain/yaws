use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::env;

enum Method {
    Get,
    Put,
    Post,
    Patch,
    Head,
    Options,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let port = &args[1];
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port {}", port);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            stream.write(&data[0..size]).unwrap();
            true
        }
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

struct Request<'a> {
    pub url: &'a str,
    pub method: Method,
}