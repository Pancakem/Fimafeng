use std::collections::HashMap;

/// HTTP header as tuple of key and value
pub type Header = (String, String);

/// HTTP request parameter map of string keys and string values
pub type Params = HashMap<String, String>;

/// HTTP headers map of string keys and string values
pub type Headers = Vec<Header>;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Method {
    Get,
    Post,
}

impl ToString for Method {
    fn to_string(&self) -> String {
        match self {
            Method::Get => "GET".to_string(),
            Method::Post => "POST".to_string(),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum HTTPVersion {
    HTTP1,
    HTTP2, // currently not supported
}

impl ToString for HTTPVersion {
    fn to_string(&self) -> String {
        match self {
            HTTPVersion::HTTP1 => "HTTP/1.1".to_string(),
            HTTPVersion::HTTP2 => "HTTP/2".to_string(),
        }
    }
}

lazy_static! {
    /// A mapping of status code and their meanings
    pub static ref STATUS_CODE_MAPPING: HashMap<u16, &'static str> = vec![
        (200, "OK"),
        (404, "Not Found"),
        (405, "Method Not Allowed"),
        (500, "Internal Server Error")
    ]
    .into_iter()
    .collect();
}
