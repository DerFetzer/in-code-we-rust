use std::sync::Arc;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let socket = Arc::new(UdpSocket::bind("127.0.0.1:11223").await?);

    let mut buf = [0; 512];

    loop {
        let (num_bytes, src) = socket.recv_from(&mut buf).await?;

        let data = buf[..num_bytes].to_vec();

        let sender = socket.clone();

        tokio::spawn(async move { sender.send_to(&data, src).await });
    }
}
