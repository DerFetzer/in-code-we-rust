use std::error::Error;
use std::net::{TcpListener, TcpStream};

fn handle_client(_stream: TcpStream) -> Result<(), Box<dyn Error>> {
    todo!("Implement me!")
}

fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:1122")?;

    for stream in listener.incoming() {
        handle_client(stream?)?;
    }
    Ok(())
}
