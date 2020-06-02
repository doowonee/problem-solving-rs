# problem-solving-rs
collection of problem solving in Rust language

# How to use

Run `cargo test` in project directory. It will be run all of unit tests. If you want to run specific test by problem then run `cargo test problem_name` for example `cargo run reverse_number`.

# If you want to see stdout

When you run `cargo test`, cargo doesn't show stdout when it's passed. run `cargo test fixed_puzzle_game -- --nocapture` will show stdout but be careful **cargo test are run by parallel** so stdout might be confused if you just run all of tests. run specific test case will be `cargo test fixed_puzzle_game::tests::unsorted_larger_than_4 -- --nocapture`

# Rule of prefix about commit message

- problem: change tests, comment, docs about quiz.
- answer: change solution logic.
- build: change cargo project structure
- etc: change README.md