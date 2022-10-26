use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

pub fn start_server() -> Result<(), &'static str> {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        // FIXME: Handle error type of stream correctly
        handle_client_connection(stream.unwrap())?;
    }

    Ok(())
}

fn handle_client_connection(mut stream: TcpStream) -> Result<(), &'static str> {
    // stream must be mut so that we can write back out to it.
    let mut buf = [0; 1024];
    match stream.read(&mut buf) {
        Ok(_) => (),
        Err(_) => return Err("Failed to stream into byte buffer"),
    };

    let http_request = String::from_utf8_lossy(&buf);

    // TODO: Remove this once we can parse out the correct information.
    eprintln!("=========\nhttp_request: {}\n========", http_request);

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

    let request_endpoint = request_endpoint(http_request_header)?;
    let request_body = request_body(&http_request);

    // FIXME: Write a properly formatted HTTP request out to the stream.
    //    let response = format!(
    //        "{}\r\nContent-Length: {}\r\n\r\n{}",
    //        status_line,
    //        body.len(),
    //        body
    //    );
    stream
        .write_fmt(format_args!(
            "HTTP/1.1 200 OK Connecting to Endpoint: {}",
            request_endpoint
        ))
        .unwrap();

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

    eprintln!("{}", http_request_header);

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

    eprintln!("endpoint: {}", endpoint);

    Ok(endpoint)
}

fn request_body(http_request: &str) -> Result<&str, &'static str> {
    // FIXME: If the message spans multiple lines this only returns the last line.
    // So no multi line messages.
    let request_body = http_request.lines().last().unwrap();
    assert!(!request_body.contains('\n'));

    Ok("fine")
}
