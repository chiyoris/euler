// https://projecteuler.net/problem=010

// cargo run -p euler_010 --release

// use euler_utils::*;
use primes::{PrimeSet, Sieve};

fn solve(limit: u64) -> u64 {
    let mut pset = Sieve::new();
    let mut sum = 0;

    for (ix, n) in pset.iter().enumerate() {
        sum += n;
        if n >= limit {
            break;
        }
    }
    sum
}

fn main() {
    let answer = solve(2_000_000);
    println!("rozwiązanie dla zadania 010: {}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(solve(), 0);
    }
}
