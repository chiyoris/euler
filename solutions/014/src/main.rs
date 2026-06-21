// https://projecteuler.net/problem=014

// cargo run -p euler_014 --release  

// use euler_utils::*;

use std::vec;

fn colatz(mut n: u64) -> Vec<u64>{
    let mut v:Vec<u64> = vec![];

    if n == 1 { return v; }

    v.push(n);

    if n % 2 == 0 { n /= 2; } else { n = 3*n + 1; }

    v.extend(colatz(n));
    return v;
}

fn solve() -> u64 {
    let mut longest = 0;
    let mut num = 0;

    for i in 1..=1_000_000{
        let arr = colatz(i);

        if arr.len() > longest{
            num = i;
            longest = arr.len();
        }
    }

    return num;
}

fn main() {
    let answer = solve();
    println!("rozwiązanie dla zadania 014: {}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(solve(), 0);
    }
}
