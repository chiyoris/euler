// https://projecteuler.net/problem=007

// cargo run -p euler_007 --release

// use euler_utils::*;
use primes::{PrimeSet, Sieve};

fn solve(n: u64) -> u64 {
    let mut pset = Sieve::new();

    pset.iter().nth(n as usize - 1).unwrap()
}

fn main() {
    let answer = solve(10_001);
    println!("rozwiązanie dla zadania 007: {}", answer);
}

// #[cfg(test)]
// mod tests {
//     use super::*;
// #[test]
// fn test_example() {
//     assert_eq!(solve(), 0);
// }
// }
