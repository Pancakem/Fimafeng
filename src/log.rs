use crate::request::Request;
use crate::response::Response;

/// Log the HTTP request.
///
/// **request** is the Request object to log.

pub fn log_request(request: &Request) {
    println!(
        "[{}] \"{} {} {}\"",
        request.time(),
        request.method().to_string(),
        request.target(),
        request.http_ver().to_string(),
    );
}

/// Log the HTTP response.
///
/// **response** is the Response object to log.

pub fn log_response(response: &Response) {
    println!(
        "[{}] \"{} \"",
        response.date.time(),
        response.status_code,
    );
}
