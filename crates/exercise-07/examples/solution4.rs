use color_eyre::eyre::{Result, WrapErr};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::{Arc, RwLock};
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tracing::{error, info, warn};

const API_URL: &str = "https://api.genderize.io";
const CACHE_FILE_NAME: &str = "cache.json";
#[derive(Debug, Serialize, Deserialize, Clone)]
struct GenderizeResponse {
    pub name: String,
    pub gender: String,
    pub probability: f64,
    pub count: i64,
}

type Cache = Arc<RwLock<HashMap<String, GenderizeResponse>>>;

async fn query_api(client: &Client, name: &str) -> Result<GenderizeResponse> {
    let res = client.get(API_URL).query(&[("name", name)]).send().await?;
    Ok(res.error_for_status()?.json().await?)
}

async fn handle_client(mut stream: TcpStream, cache: Cache) -> Result<()> {
    let (rx, mut tx) = stream.split();
    let buf_reader = BufReader::new(rx);
    let mut lines = buf_reader.lines();
    let client = reqwest::Client::new();
    while let Ok(Some(line)) = lines.next_line().await {
        match line.trim() {
            "q" => break,
            name => {
                info!("New query: {name}");
                let cached_response = {
                    let lock = cache.read().unwrap();
                    lock.get(name).cloned()
                };
                let response = if let Some(gender_respone) = cached_response {
                    info!("Cache hit: {}", name);
                    gender_respone
                } else {
                    match query_api(&client, name).await {
                        Ok(response) => {
                            cache
                                .write()
                                .unwrap()
                                .insert(name.to_string(), response.clone());
                            if let Err(err) = write_cache(&cache).await {
                                warn!("Could not write cache: {err}");
                            }
                            response
                        }
                        Err(e) => {
                            tx.write_all(format!("Error: {e}\n").as_bytes())
                                .await
                                .wrap_err("Could not write to stream")?;
                            continue;
                        }
                    }
                };
                tx.write_all(format!("{}\n", response.gender).as_bytes())
                    .await
                    .wrap_err("Could not write to stream")?;
            }
        }
    }
    Ok(())
}

async fn write_cache(cache: &Cache) -> Result<()> {
    let mut file = File::create(CACHE_FILE_NAME).await?;
    let file_content = serde_json::to_string_pretty(cache.read().unwrap().deref())?;
    file.write_all(file_content.as_bytes()).await?;
    Ok(())
}

fn read_cache() -> Result<Cache> {
    std::fs::File::open(CACHE_FILE_NAME)
        .wrap_err("Could not open cache file")
        .and_then(|mut f| {
            let mut dst = String::new();
            std::io::Read::read_to_string(&mut f, &mut dst)
                .wrap_err("Could not read from cache file")?;
            Ok(dst)
        })
        .and_then(|s| serde_json::from_str(s.as_str()).wrap_err("Cache file content is not valid"))
        .map(|c| Arc::new(RwLock::new(c)))
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    console_subscriber::init();

    let cache: Cache = match read_cache() {
        Ok(c) => c,
        Err(e) => {
            warn!("Could not read cache: {e}");
            Default::default()
        }
    };

    let listener = TcpListener::bind("127.0.0.1:1122").await?;
    loop {
        let (stream, _) = listener.accept().await?;
        let cache = cache.clone();
        tokio::task::spawn(async move {
            if let Err(e) = handle_client(stream, cache).await {
                error!("Error while handling client: {e}")
            }
        });
    }
}
