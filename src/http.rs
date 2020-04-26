use std::io::{Read, BufReader};
use std::io::*;

#[derive(Debug)]
pub enum HttpMethod {
    Get,
    Put,
    Post,
    Patch,
    Head,
    Options,
    Unknown,
}

pub struct Response<'a> {
    pub result: &'a [u8]
}

pub struct Request {
    pub request_meta: RequestMeta,
    pub request_stream: Box<dyn Read>,
}

#[derive(Debug)]
pub struct RequestMeta {
    pub method: HttpMethod,
    pub version: String,
    pub uri: String,
}


pub struct HandlerError {}


pub fn get_http_meta(line: &str) -> Option<RequestMeta> {
    let splits: Vec<&str> = line.split(" ").collect();
    let method_string = splits[0];
    let path = splits[1];
    let version_splits: Vec<&str> = splits[2].split("HTTP/").collect();
    let method = match method_string.to_ascii_lowercase().as_str() {
        "get" => HttpMethod::Get,
        "post" => HttpMethod::Post,
        "put" => HttpMethod::Put,
        "patch" => HttpMethod::Patch,
        "head" => HttpMethod::Head,
        "options" => HttpMethod::Options,
        _ => HttpMethod::Unknown
    };
    Some(RequestMeta {
        method,
        uri: String::from(path),
        version: String::from(version_splits[1].trim()),
    })
}

pub fn handler<'a>(request: Request) -> std::result::Result<Response<'a>, HandlerError> {
    let mut buffer = String::new();
    let mut reader = BufReader::new(request.request_stream);

    Ok(Response { result: "Success".as_bytes() })
}