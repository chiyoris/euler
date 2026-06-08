#!/bin/bash

NUM="$1"

mkdir -p "solutions/$NUM/src"

cat <<-CARGO_EOF > "solutions/$NUM/Cargo.toml"
[package]
name = "euler_${NUM}"
version = "0.1.0"
edition = "2021"

[dependencies]
euler_utils = { path = "../../euler_utils" }
CARGO_EOF

cat <<-RS_EOF > "solutions/$NUM/src/main.rs"
// cargo run -p euler_${NUM} --release  

// use euler_utils::*;

fn solve() -> u64 {
    0
}

fn main() {
    let answer = solve();
    println!("rozwiązanie dla zadania ${NUM}: {}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(solve(), 0);
    }
}
RS_EOF

echo "✅ utworzono szablon dla zadania ${NUM}!"
