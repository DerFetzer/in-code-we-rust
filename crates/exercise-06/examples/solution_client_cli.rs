use clap::Parser;
use std::net::UdpSocket;

/// Awesome UDP client
#[derive(Parser, Debug)]
#[clap(name = "udp-cli", author, version, long_about = None)]
struct Args {
    /// Local address to bind to
    #[arg(long, default_value = "0.0.0.0")]
    bind_address: String,

    /// Local port to bind to
    #[arg(long)]
    bind_port: Option<u16>,

    /// Destination address to bind to
    #[arg()]
    destination_address: String,

    /// Destination port to bind to
    #[arg()]
    destination_port: u16,
}

fn main() -> std::io::Result<()> {
    let args: Args = Args::parse();

    let socket = UdpSocket::bind(format!(
        "{}:{}",
        args.bind_address,
        args.bind_port.unwrap_or(0),
    ))?;
    socket.connect(format!(
        "{}:{}",
        args.destination_address, args.destination_port
    ))?;

    let mut buf = [0; 512];

    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line)?;

        socket.send(line.as_bytes())?;

        let num_bytes = socket.recv(&mut buf)?;
        print!("{}", std::str::from_utf8(&buf[..num_bytes]).unwrap());
    }
}
