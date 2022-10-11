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

    if !is_valid_http_request(&buf) {
        stream
            .write(b"ERROR: Invalid HTTP request!\n")
            .expect("Failed to write to stream.");
    }

    Ok(())
}

fn is_valid_http_request(buf: &[u8; 1024]) -> bool {
    let request_content = String::from_utf8_lossy(buf);

    // <REQUEST TYPE> <URI> HTTP/1.1
    let http_request_header = request_content
        .lines()
        .next()
        .expect("Failed to get iterator over lines.");

    // Only accept GET and PUT requests to the server, that is all that people
    // should need to do with an IRC chat.
    if !http_request_header.ends_with("HTTP/1.1")
        || !(http_request_header.starts_with("PUT") || http_request_header.starts_with("GET"))
    {
        // FIXME: Write this to stream.
        eprintln!("Invalid HTTP request");
        return false;
    }

    eprintln!("{}", request_content);

    true
}
