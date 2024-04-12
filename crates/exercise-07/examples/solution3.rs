use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, RwLock};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

const API_URL: &str = "https://api.genderize.io";

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GenderizeResponse {
    pub name: String,
    pub gender: String,
    pub probability: f64,
    pub count: i64,
}

type Cache = Arc<RwLock<HashMap<String, GenderizeResponse>>>;

async fn query_api(client: &Client, name: &str) -> Result<GenderizeResponse, reqwest::Error> {
    let res = client.get(API_URL).query(&[("name", name)]).send().await?;
    res.error_for_status()?.json().await
}

async fn handle_client(mut stream: TcpStream, cache: Cache) {
    let (rx, mut tx) = stream.split();
    let buf_reader = BufReader::new(rx);
    let mut lines = buf_reader.lines();
    let client = reqwest::Client::new();
    while let Ok(Some(line)) = lines.next_line().await {
        match line.trim() {
            "q" => break,
            name => {
                println!("New query: {name}");
                let cached_response = {
                    let lock = cache.read().unwrap();
                    lock.get(name).cloned()
                };
                let response = if let Some(gender_respone) = cached_response {
                    println!("Cache hit: {}", name);
                    gender_respone
                } else {
                    match query_api(&client, name).await {
                        Ok(response) => {
                            cache
                                .write()
                                .unwrap()
                                .insert(name.to_string(), response.clone());
                            response
                        }
                        Err(e) => {
                            tx.write_all(format!("Error: {e}\n").as_bytes())
                                .await
                                .unwrap();
                            continue;
                        }
                    }
                };
                tx.write_all(format!("{}\n", response.gender).as_bytes())
                    .await
                    .unwrap();
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    console_subscriber::init();

    let cache: Cache = Cache::default();

    let listener = TcpListener::bind("127.0.0.1:1122").await?;
    loop {
        let (stream, _) = listener.accept().await?;
        let cache = cache.clone();
        tokio::task::spawn(async move { handle_client(stream, cache).await });
    }
}
