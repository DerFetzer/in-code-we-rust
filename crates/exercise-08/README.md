# `exercise-08`
This exercise and the ones following are different from the ones before.
Head over to the repo [result-guide](https://github.com/DerFetzer/result-guide) clone it and checkout branch `01-intro`.

## ToDo
1. Install `sea-orm-cli` with `cargo install sea-orm-cli`.
2. Have a look at [SeaORM](https://www.sea-ql.org/SeaORM/) and find out what it does.
3. Have a look at the existing source code in `result-guide` and the [SeaORM tutorial](https://www.sea-ql.org/sea-orm-tutorial/ch00-00-introduction.html) to find out what is already implemented.
4. Run the program and inspect the generated database.
5. Add a migration for a new table `teststep` that holds each test step of a report.
6. Generate database entities using `sea-orm-client` with `sea-orm-cli generate entity -u sqlite:./sqlite.db?mode=rwc -o src/entities`.

## Resources
- https://www.sea-ql.org/SeaORM/docs/index/
- https://tokio.rs/
