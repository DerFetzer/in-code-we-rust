# `exercise-03`
Goal of this exercise is to implement a simple library system where you can lend and give back books.
The core type `HashMap` will play an important role.

## ToDo
1. Have a look at the `Library` struct in [lib.rs](src/lib.rs) and an example of its intended use in [main.rs](src/main.rs).
2. Implement the unit tests `fill_stock` and `lend_take_back`.
3. Replace all `todo()` calls to have all unit tests and the main program run successfully. Think about what could be a good return type for `get_books`.
4. Add a transaction log to the `Library` and print all transactions to the terminal.

## Resources
- https://doc.rust-lang.org/stable/std/collections/hash_map/struct.HashMap.html
- https://doc.rust-lang.org/stable/std/collections/hash_map/struct.HashMap.html#method.entry
- https://doc.rust-lang.org/stable/std/iter/index.html
- https://doc.rust-lang.org/stable/core/iter/trait.ExactSizeIterator.html
