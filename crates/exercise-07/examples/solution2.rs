use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

const API_URL: &str = "https://api.genderize.io";

#[derive(Serialize, Deserialize)]
struct GenderizeResponse {
    pub name: String,
    pub gender: String,
    pub probability: f64,
    pub count: i64,
}

async fn query_api(client: &Client, name: &str) -> Result<GenderizeResponse, reqwest::Error> {
    let res = client.get(API_URL).query(&[("name", name)]).send().await?;
    res.error_for_status()?.json().await
}

async fn handle_client(mut stream: TcpStream) {
    let (rx, mut tx) = stream.split();
    let buf_reader = BufReader::new(rx);
    let mut lines = buf_reader.lines();
    let client = reqwest::Client::new();
    while let Ok(Some(line)) = lines.next_line().await {
        match line.trim() {
            "q" => break,
            name => {
                println!("New query: {name}");
                match query_api(&client, name).await {
                    Ok(response) => tx
                        .write_all(format!("{}\n", response.gender).as_bytes())
                        .await
                        .unwrap(),
                    Err(e) => tx
                        .write_all(format!("Error: {}\n", e.to_string()).as_bytes())
                        .await
                        .unwrap(),
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    console_subscriber::init();

    let listener = TcpListener::bind("127.0.0.1:1122").await?;
    loop {
        let (stream, _) = listener.accept().await?;
        tokio::task::spawn(async move { handle_client(stream).await });
    }
}
