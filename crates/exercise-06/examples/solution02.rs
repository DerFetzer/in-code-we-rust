use std::net::{SocketAddr, UdpSocket};
use std::sync::Arc;

fn main() -> std::io::Result<()> {
    let socket = Arc::new(UdpSocket::bind("127.0.0.1:11223")?);
    let sender = socket.clone();

    let (tx, rx) = std::sync::mpsc::channel::<(Vec<u8>, SocketAddr)>();

    std::thread::spawn(move || {
        while let Ok((bytes, addr)) = rx.recv() {
            sender.send_to(&bytes, addr).unwrap();
        }
    });

    let mut buf = [0; 512];

    loop {
        let (num_bytes, src) = socket.recv_from(&mut buf)?;

        let data = buf[..num_bytes].to_vec();

        tx.send((data, src)).unwrap();
    }
}
