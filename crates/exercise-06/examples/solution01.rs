use std::net::UdpSocket;
use std::sync::Arc;

fn main() -> std::io::Result<()> {
    let socket = Arc::new(UdpSocket::bind("127.0.0.1:11223")?);

    let mut buf = [0; 512];

    loop {
        let (num_bytes, src) = socket.recv_from(&mut buf)?;

        let data = buf[..num_bytes].to_vec();

        let socket = socket.clone();

        std::thread::spawn(move || {
            socket.send_to(&data, src).unwrap();
        });
    }
}
