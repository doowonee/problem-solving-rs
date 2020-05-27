//! This is main file but there is nothing to running.
//! use `cargo test` to test each problems.

// https://doc.rust-lang.org/book/ch11-03-test-organization.html#the-tests-module-and-cfgtest
// https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html#separating-modules-into-different-files
#[cfg(test)]
mod reverse_number;
#[cfg(test)]
mod puzzle;

fn main() {
    println!("This is not a program. Please run cargo test to use this repo.");
}