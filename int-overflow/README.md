A little project to demonstrate how Rust handles int overflows in debug and release builds.

To use:

- run `cargo build && cargo build --release`
- run `./target/debug/int-overflow`
- add a number larger greater than 5
- observe panic described in the Data Types section of the Rust Book (3.2)

- run `./target/release/int-overflow`
- observe the "two’s complement wrapping" behavior described in the Data Types section of the Rust Book (3.2)
