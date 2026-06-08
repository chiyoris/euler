#!/bin/bash

NUM="002"

mkdir -p "solutions/$NUM/src"

# Użyj <<-EOF żeby tolerować wcięcia (tabulatory)
cat <<-CARGO_EOF > "solutions/$NUM/Cargo.toml"
[package]
name = "euler_${NUM}"
version = "0.1.0"
edition = "2021"

[dependencies]
euler_utils = { path = "../../euler_utils" }
CARGO_EOF

cat <<-RS_EOF > "solutions/$NUM/src/main.rs"
// Użycie wspólnej biblioteki (odkomentuj jeśli potrzebne):
// use euler_utils::*;

fn solve() -> u64 {
    // Miejsce na Twoje rozwiązanie
    0
}

fn main() {
    let answer = solve();
    println!("Rozwiązanie dla zadania ${NUM}: {}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        // Tutaj wpisz przykładowy test z treści zadania
        assert_eq!(solve(), 0);
    }
}
RS_EOF

echo "✅ Pomyślnie utworzono szablon dla zadania ${NUM}!"
