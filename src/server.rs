use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

pub fn start_server() -> Result<(), &'static str> {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        match handle_client_connection(stream.unwrap()) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };
    }

    Ok(())
}

fn handle_client_connection(mut stream: TcpStream) -> Result<(), &'static str> {
    let mut buf = [0; 1024];
    match stream.read(&mut buf) {
        Ok(_) => (),
        Err(_) => return Err("Failed to stream into byte buffer"),
    };

    // FIXME: This should really be exported to its own function but it is a
    // pain to do it without doing it really slowly. Easy to just create vectors
    // and buffers all over but we don't want that.
    let mut count = 0;
    for byte in buf.into_iter().rev() {
        if byte != 0 {
            break;
        }
        count += 1;
    }
    let buf = &buf[0..buf.len() - count];

    let http_request = String::from_utf8_lossy(&buf);

    // <REQUEST TYPE> <URI> HTTP/1.1
    let http_request_header = http_request
        .lines()
        .next()
        .expect("Failed to get iterator over lines.");

    if !is_valid_http_request(http_request_header) {
        stream
            .write_all(b"ERROR: Invalid HTTP request!\n")
            .expect("Failed to write to stream.");
    }

    let request_endpoint = request_endpoint(&http_request_header)?;
    let request_body = request_body(&http_request)?;
    let request_body = request_body.trim();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        "HTTP/1.1 200 OK",
        request_body.len(),
        request_body
    );

    match stream.write(response.as_bytes()) {
        Ok(_) => (),
        Err(_) => return Err("Failed to write to stream. Try again."),
    };
    stream.flush().expect("Failed to flush stream.");

    Ok(())
}

fn is_valid_http_request(http_request_header: &str) -> bool {
    // Only accept GET and PUT requests to the server, that is all that people
    // should need to do with an IRC chat.
    if !http_request_header.ends_with("HTTP/1.1")
        || !(http_request_header.starts_with("PUT") || http_request_header.starts_with("GET"))
    {
        // FIXME: Write this to stream.
        eprintln!("Invalid HTTP request");
        return false;
    }

    true
}

fn request_endpoint(http_request_header: &str) -> Result<&str, &'static str> {
    let mut split_http_header = http_request_header.split_whitespace();
    split_http_header.next().unwrap();
    let endpoint = split_http_header
        .next()
        .expect("Split HTTP request failed.");

    // Endpoints come in as /<endpoint>, remove the '/'
    let endpoint = endpoint.trim_start_matches('/');

    Ok(endpoint)
}

fn request_body(http_request: &str) -> Result<String, &'static str> {
    let request_body = match http_request.split('\n').last() {
        Some(body) => body,
        None => return Err("Failed to parse body from request."),
    };

    let request_body = request_body.trim().to_string();

    Ok(request_body)
}

#[cfg(test)]
mod test {
    use super::*;

    pub const REQUEST_HEADER: &str = "PUT /systems HTTP/1.1";
    pub const HTTP_REQUEST: &str = "\
PUT /test HTTP/1.1
Host: localhost:8080
User-Agent: curl/7.81.0
Accept: */*
Content-Length: 4
Content-Type: application/x-www-form-urlencoded

test";

    #[test]
    fn test_request_endpoint() {
        assert_eq!(request_endpoint(REQUEST_HEADER).unwrap(), "systems");
    }

    #[test]
    fn test_request_body() {
        assert_eq!(request_body(HTTP_REQUEST).unwrap(), "test");
    }
}
