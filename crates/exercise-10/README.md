# `exercise-10`
This exercise is about implementing iterators for custom structs.

## ToDo
1. Implement an iterator for `Tuple` that iterates by reference.
2. Implement `IntoIterator` for `&Tuple` using the implementation from the step above.
3. Do steps 1 und 2 for iterating by value as well as by mutable reference. (Hint: use [std::mem::take](https://doc.rust-lang.org/std/mem/fn.take.html))
4. Implement `DoubleEndedIterator` for all the iterators.
5. Have at look at (solution3)[examples/solution3] and try to understand what it does.

## Resources
- https://doc.rust-lang.org/std/iter/index.html
- https://doc.rust-lang.org/std/iter/trait.Iterator.html
- https://doc.rust-lang.org/std/iter/trait.IntoIterator.html
- https://doc.rust-lang.org/std/iter/trait.DoubleEndedIterator.html
