//! The tests/ directory holds integration tests. These can test either your library or a binary.
//! When you run `cargo test`, cargo will ensure that your library and all binaries have been built
//! prior to running the integration tests.
//!
//! Here we're testing the library. Since this is an integration test, we use our library just
//! like we would if we were depending on any other crate from crates.io (including the `extern crate`)

use testing_rust::factorial;

#[test]
fn factorial_3_returns_6() {
    assert_eq!(6, factorial(3));
}
