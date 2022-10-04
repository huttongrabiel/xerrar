use xerrar::server;

fn main() -> Result<(), &'static str> {
    match server::start_server() {
        Ok(_) => (),
        Err(e) => return Err(e),
    };

    Ok(())
}
