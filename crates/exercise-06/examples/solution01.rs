use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:11223")?;

    let mut buf = [0; 512];

    loop {
        let (num_bytes, src) = socket.recv_from(&mut buf)?;

        let socket = socket.try_clone()?;

        std::thread::spawn(move || {
            socket.send_to(&buf[..num_bytes], src).unwrap();
        });
    }
}
