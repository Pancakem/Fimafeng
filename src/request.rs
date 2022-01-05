use crate::http::{HTTPVersion, Header, Headers, Method, Params};
use crate::parser::{
    parse_http_headers, parse_http_params, parse_http_version, parse_method, parse_request_target,
};
use anyhow::Error;

#[derive(Debug)]
pub struct Request {
    method: Method,
    params: Params,
    headers: Headers,
    path: String,
    http_version: HTTPVersion,
    body: Option<String>,
}

impl TryFrom<&str> for Request {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (remaining_input, method) = parse_method(value).unwrap();
        let (remaining_input, path) = parse_request_target(remaining_input).unwrap();
        let (remaining_input, params) = parse_http_params(remaining_input).unwrap();
        let (remaining_input, http_version) = parse_http_version(remaining_input).unwrap();

        let (remaining_input, headers) = parse_http_headers(remaining_input).unwrap();

        let mut body : Option<String> = None;
        if method == Method::POST {
            body = Some(remaining_input.to_string());
        }

        Ok(Self {
            method,
            path: path.to_string(),
            http_version,
            params,
            headers,
            body,
        })
    }
}

impl Request {
    pub fn method(&self) -> Method {
        self.method
    }

    pub fn get_header(_header_name: &str) -> Header {
        todo!()
    }

    pub fn target(&self) -> String {
        self.path.clone()
    }

    pub fn body(&self) -> Option<&String> {
        self.body.as_ref()
    }

    pub fn http_ver(&self) -> HTTPVersion {
        self.http_version.clone()
    }

    pub fn get_param(_param_name: &str) -> String {
        todo!()
    }

}
