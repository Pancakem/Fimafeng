use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_till},
    character::complete::{alphanumeric0, alphanumeric1},
    error::VerboseError,
    multi::many0,
    sequence::{pair, preceded, separated_pair, terminated},
    IResult,
};

use crate::http::{HTTPVersion, Headers, Method, Params};

pub fn parse_method(input: &str) -> IResult<&str, Method, VerboseError<&str>> {
    let methods: Vec<&str> = vec!["POST", "GET"];

    for method in methods {
        let res: Result<(&str, &str), nom::Err<VerboseError<&str>>> = tag(method)(input);
        match res {
            Ok((i, _)) => {
                let m = match method {
                    "POST" => Method::Post,
                    "GET" => Method::Get,
                    _ => unreachable!(),
                };
                return Ok((i, m));
            }
            Err(_) => continue,
        };
    }

    let e = nom::Err::Incomplete(nom::Needed::new(0));
    Err(e)
}

pub fn parse_request_target(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    let input = input.trim_start();
    is_not(" \t\r\n?")(input)
}

// support HTTP version 1 only
pub fn parse_http_version(input: &str) -> IResult<&str, HTTPVersion, VerboseError<&str>> {
    let input = input.trim_start();
    match terminated(tag("HTTP/1.1"), tag("\r\n"))(input) {
        Ok((i, _)) => Ok((i, HTTPVersion::HTTP1)),
        Err(e) => Err(e),
    }
}

// parses the request parameters
// returns a map empty if no parameters were passed
pub fn parse_http_params(input: &str) -> IResult<&str, Params, VerboseError<&str>> {
    let mut params = Params::new();
    let res: Result<(&str, Vec<(&str, &str)>), nom::Err<VerboseError<&str>>> = preceded(
        tag("?"),
        many0(separated_pair(
            alphanumeric1,
            tag("="),
            alt((terminated(alphanumeric0, tag("&")), alphanumeric0)),
        )),
    )(input);

    let (remaining_input, list) = match res {
        Ok(r) => r,
        Err(_) => return Ok((input, params)),
    };

    for (k, v) in list {
        params.insert(k.to_string(), v.to_string());
    }

    Ok((remaining_input, params))
}

// parses http headers
// goes through lines of headers until can't match any headers
pub fn parse_http_headers(input: &str) -> IResult<&str, Headers, VerboseError<&str>> {
    let mut headers = Headers::new();

    let res: Result<(&str, Vec<(&str, &str)>), nom::Err<VerboseError<&str>>> = many0(pair(
        take_till(|c| c == ':'),
        preceded(tag(": "), terminated(is_not("\r\n"), tag("\r\n"))),
    ))(input);

    let (rest_input, res) = res.unwrap();

    for (k, v) in res {
        headers.insert(k.to_string(), v.to_string());
    }
    Ok((rest_input, headers))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::{HTTPVersion, Method, Params};
    use std::collections::HashMap;
    use std::hash::Hash;

    #[test]
    fn test_parse_method() {
        assert_eq!(parse_method("POST"), Ok(("", Method::Post)));
        assert_eq!(parse_method("GET"), Ok(("", Method::Get)));
    }

    #[test]
    fn test_parse_request_target() {
        assert_eq!(
            parse_request_target("/accounts/login "),
            Ok((" ", "/accounts/login"))
        )
    }

    #[test]
    fn test_parse_http_version() {
        assert_eq!(
            parse_http_version("HTTP/1.1\r\n"),
            Ok(("", HTTPVersion::HTTP1))
        );
    }

    #[test]
    fn test_parse_params() {
        match parse_http_params("?test=hello&base=1") {
            Ok((_, map)) => {
                let mut params = Params::new();
                params.insert("test".to_string(), "hello".to_string());
                params.insert("base".to_string(), "1".to_string());
                assert!(keys_match(&map, &params));
            }
            Err(_e) => return,
        };
    }

    #[test]
    fn test_parse_headers() {
        match parse_http_headers("Host: 127.0.0.1\r\nUser-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.9; rv:50.0) Gecko/20100101 Firefox/50.0\r\n") {
                Ok((_, map)) => {
                    let mut headers = Headers::new();
                    headers.insert("Host".to_string(), "".to_string());
                    headers.insert("User-Agent".to_string(), "".to_string());
                    assert!(keys_match(&map, &headers));
                },
                Err(_e) => return,
            };
    }

    fn keys_match<T: Eq + Hash, U, V>(map1: &HashMap<T, U>, map2: &HashMap<T, V>) -> bool {
        map1.len() == map2.len() && map1.keys().all(|k| map2.contains_key(k))
    }
}
