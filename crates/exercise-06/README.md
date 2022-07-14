# `exercise-06`
This exercise is about simple network programming and writing a CLI using [clap](https://docs.rs/clap/latest/clap/index.html).

## ToDo
1. Run the existing UDP echo server and test it with something like `nc`. What could be problematic with the implementation.
2. Implement an improved version that spawns a thread for each incoming UPD packet. What could be problematic with this implementation.
3. Implement a version that uses message passing and only one thread for sending.
4. Implement a client for our echo server in [client.rs](src/bin/client.rs) that reads user input from `stdin`.
5. Implement a CLI for the client using [clap](https://docs.rs/clap/latest/clap/index.html).

## Resources
- https://doc.rust-lang.org/std/net/struct.UdpSocket.html
- https://docs.rs/clap/latest/clap/index.html
