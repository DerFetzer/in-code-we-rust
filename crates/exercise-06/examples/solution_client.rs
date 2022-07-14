use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:22334")?;
    socket.connect("127.0.0.1:11223")?;

    let mut buf = [0; 512];

    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line)?;

        socket.send(line.as_bytes())?;

        let num_bytes = socket.recv(&mut buf)?;
        println!("{}", std::str::from_utf8(&buf[..num_bytes]).unwrap());
    }
}
