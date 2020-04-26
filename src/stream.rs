use std::net::TcpStream;
use std::thread;
use std::io::*;
use crate::http::*;

pub fn handle_request(mut stream: std::io::Result<TcpStream>) {
    match stream {
        Ok(stream) => {
            thread::spawn(move || handle_connection(stream));
        }
        Err(e) => println!("Error: {}", e)
    }
}


pub fn handle_connection(mut stream: TcpStream) {
    let mut reader = BufReader::new(&stream);
    let mut buffer = String::new();
    match reader.read_line(&mut buffer) {
        Ok(_) => if let Some(meta) = get_http_meta(&buffer) {
            let rest = Box::new(stream.try_clone().unwrap());
            if let Ok(mut response) = handler(Request {
                request_meta: meta,
                request_stream: rest,
            }) {
                let mut result: Vec<u8> = vec![];
                result.extend_from_slice("HTTP/1.1 200 OK\r\n\r\n".as_bytes());
                result.extend_from_slice(response.result);
                stream.write(&result[..]).unwrap();
                stream.flush().unwrap();
            }
        }
        Err(_) => {
            let mut result: Vec<u8> = vec![];
            result.extend_from_slice("HTTP/1.1 500 Internal Server Error\r\n\r\n".as_bytes());
            stream.write(&result[..]).unwrap();
            stream.flush().unwrap();
        }
    }
}


