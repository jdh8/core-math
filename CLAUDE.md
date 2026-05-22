# core-math

This crate provides a Rusty API over [`core-math-sys`](https://crates.io/crates/core-math-sys),
exposing CORE-MATH's correctly rounded mathematical functions.  Correct rounding is
the theoretical accuracy and beats most old C libraries such as glibc.

After updating the codebase, please

- Format the code with `cargo fmt`.
- Run the tests with `cargo test --all-features`.
- Update [CHANGELOG.md](CHANGELOG.md) with a summary of the changes and their impact on users.
- Propose a clear and descriptive commit message.
