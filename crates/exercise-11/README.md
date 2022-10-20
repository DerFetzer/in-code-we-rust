# `exercise-11`
Goal of this exercise is to add Tests for all API-Endpoints and to improve error handling. 
Check out branch `03-test_and_error` of the [result-guide](https://github.com/DerFetzer/result-guide) repo.

## ToDo
1. What has to be done to make the endpoints testable?
2. Have a look at this [example](https://github.com/tokio-rs/axum/blob/axum-v0.5.16/examples/testing/src/main.rs) and implement endpoint tests accordingly. (Hint: For simplicity do not mock the database.)
3. Error handling is tedious at the moment since we are not using the `?` operator. Find out how [anyhow](https://github.com/dtolnay/anyhow) or [eyre](https://github.com/yaahc/eyre) can help here.
4. Implement error handling using `eyre`. (Hint: If you do not find a solution right away have a look at [this](https://github.com/tokio-rs/axum/blob/7caa4a3a47a31c211d301f3afbc518ea2c07b4de/examples/anyhow-error-response/src/main.rs) and [this](https://github.com/DerFetzer/eps-server/blob/d55b4d9029271b7fe2983a5e08c9ca244b7982a3/src/error.rs))

## Resources
- https://docs.rs/axum/latest/axum/
- https://github.com/dtolnay/anyhow
- https://github.com/yaahc/eyre
