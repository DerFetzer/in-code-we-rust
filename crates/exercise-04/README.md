# `exercise-04`
This exercise is about parallelism using threads.
The task is to count all unique words in the bible. A German machine-readable version is downloaded on first run.

## ToDo
1. Have a look at the provided functions in [lib.rs](src/lib.rs).
2. Implement the parallel parsing using shared state, e.g. `Arc<Mutex<?>>`.
3. Implement the parallel parsing using message passing, e.g. `mpsc`.
4. Bonus: Implement the parallel parsing using [rayon](https://docs.rs/rayon/1.5.3/rayon/).

## Resources
- https://doc.rust-lang.org/stable/std/thread/index.html
- https://doc.rust-lang.org/stable/std/sync/index.html
- https://doc.rust-lang.org/stable/std/sync/struct.Arc.html
- https://doc.rust-lang.org/stable/std/sync/struct.Mutex.html
- https://doc.rust-lang.org/stable/std/sync/mpsc/index.html
- https://docs.rs/rayon/1.5.3/rayon/
