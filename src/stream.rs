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
    reader.read_line(&mut buffer);
    let option = get_http_meta(&buffer);
    println!("{:?}", option);
    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
