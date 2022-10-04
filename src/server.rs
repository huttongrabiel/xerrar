use std::{
    io::Read,
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

    eprintln!("{:?}", buf);

    Ok(())
}
