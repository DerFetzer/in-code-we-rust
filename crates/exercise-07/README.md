# `exercise-06`
This exercise is about `async` Rust using [tokio](https://docs.rs/tokio/latest/tokio/index.html).
Goal of this exercise is to write an async TCP server that queries the [genderize.io API](https://genderize.io/).

## ToDo
1. Implement a blocking version starting from the existing [main.rs](src/main.rs) that reads lines from the socket and either queries the API when it receives a name and returns the gender prediction or terminates the connection when it receives "q".

## Resources
- https://rust-lang.github.io/async-book
- https://tokio.rs/
- https://docs.rs/tokio/1.20.1/tokio/
