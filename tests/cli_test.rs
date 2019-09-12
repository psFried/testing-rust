//! The tests/ directory holds integration tests. These can test either your library or a binary.
//! When you run `cargo test`, cargo will ensure that your library and all binaries have been built
//! prior to running the integration tests.
//!
//! Here we're using assert_cmd to test the command line interface of our main binary
//! https://github.com/assert-rs/assert_cmd
//! https://rust-lang-nursery.github.io/cli-wg/tutorial/testing.html

use std::process::Command;
use assert_cmd::prelude::{CommandCargoExt, OutputAssertExt};

#[test]
fn prints_the_output_of_factorial(){
    let mut command = Command::cargo_bin("factorial").unwrap();
    command.args(&["6"]).assert().success().stdout("720\n");
}

#[test]
fn prints_an_error_when_argument_is_not_a_number() {
    let mut command = Command::cargo_bin("factorial").unwrap();
    command
        .env_remove("RUST_BACKTRACE") // ensure that the whole backtrace doesn't get printed
        .args(&["foo"]) // provide a non-integer argument
        .assert()       // assert() function provided by the `OutputAssertExt` trait
            .failure()
            .code(1)
            .stdout("input is not a valid integer\n");
}
