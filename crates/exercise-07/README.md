# `exercise-06`
This exercise is about `async` Rust using [tokio](https://docs.rs/tokio/latest/tokio/index.html).
Goal of this exercise is to write an `async` TCP server that queries the [genderize.io API](https://genderize.io/).

## ToDo
1. Implement a blocking version starting from the existing [main.rs](src/main.rs) that reads lines from the socket and either queries the [genderize.io API](https://genderize.io/) when it receives a name and returns the gender prediction or terminates the connection when it receives "q".
2. What could be the problem when there is more than one client connected at the same time?
3. Make yourself comfortable with `async` using the links below and implement an `async` version using [tokio](https://docs.rs/tokio/latest/tokio/index.html) that solves the problem.
4. Since the [genderize.io API](https://genderize.io/) is limited to 1000 requests per day implement an in-memory cache for the responses.
5. [*Optional*] Make the cache persistent.
6. [*Optional*] Support all query parameters of the [genderize.io API](https://genderize.io/) and report all the response parameters back to the client.
7. [*Optional*] Use [tokio console](https://github.com/tokio-rs/console#running-the-console) to Monitor the state of the server or set `RUST_LOG=trace` to print tracing information to `stdout`. (Prerequisites are already satisfied in the solutions.)

## Resources
- https://rust-lang.github.io/async-book
- https://tokio.rs/
- https://docs.rs/tokio/1.20.1/tokio/
- https://github.com/tokio-rs/console
