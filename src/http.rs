use std::collections::HashMap;


pub type Header = (String, String);

pub type Params = HashMap<String, String>;

pub type Headers = HashMap<String, String>;


#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Method {
    GET, POST,
}

impl ToString for Method{
    fn to_string(&self) -> String {
        match self {
            Method::GET => "GET".to_string(),
            Method::POST => "POST".to_string(),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum HTTPVersion {
    HTTP1,
    // HTTP2 Not supported
}

impl ToString for HTTPVersion{
    fn to_string(&self) -> String {
        match self {
            HTTPVersion::HTTP1 => "HTTP/1.1".to_string(),
            _ => unreachable!(),
        }
    }
}

lazy_static! {
    pub static ref STATUS_CODE_MAPPING: HashMap<u16, &'static str> = vec![
        (200, "OK"),
        (404, "Not Found"),
    ].into_iter().collect();
}