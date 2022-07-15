use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:11223")?;

    let mut buf = [0; 512];

    loop {
        let (num_bytes, src) = socket.recv_from(&mut buf)?;

        socket.send_to(&buf[..num_bytes], src)?;
    }
}
