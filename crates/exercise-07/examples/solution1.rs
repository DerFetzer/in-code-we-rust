use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

const API_URL: &str = "https://api.genderize.io";

#[derive(Serialize, Deserialize)]
struct GenderizeResponse {
    pub name: String,
    pub gender: String,
    pub probability: f64,
    pub count: i64,
}

fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let buf_reader = BufReader::new(stream.try_clone()?);
    let client = reqwest::blocking::Client::new();
    for line in buf_reader.lines() {
        match line?.trim() {
            "q" => break,
            name => {
                println!("{name}");
                let res = client.get(API_URL).query(&[("name", name)]).send()?;
                let response: GenderizeResponse = res.error_for_status()?.json()?;
                stream.write_all(format!("{}\n", response.gender).as_bytes())?;
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:1122")?;

    for stream in listener.incoming() {
        handle_client(stream?)?;
    }
    Ok(())
}
