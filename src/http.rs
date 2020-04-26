use std::io::Read;

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

pub struct Request<'a> {
    pub uri: &'a str,
    pub method: HttpMethod,
    pub reader: Box<dyn Read>,
}

#[derive(Debug)]
pub struct RequestMeta {
    pub method: HttpMethod,
    pub version: String,
    pub uri: String,
}

pub struct Response {}

pub struct HandlerError {}

pub type Handler = dyn FnOnce(Request) -> std::result::Result<Response, HandlerError>;

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