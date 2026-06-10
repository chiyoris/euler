// https://projecteuler.net/problem=005

// cargo run -p euler_005 --release

// use euler_utils::*;

fn solve() -> u64 {
    let primes: [u64; 8] = [2, 3, 5, 7, 11, 13, 17, 19];

    let mut prod = 1;

    for p in primes {
        let mut n: u32 = 2;
        prod *= p;

        while p.pow(n) < 21 {
            prod *= p;
            n += 1;
        }
    }

    prod
}

fn main() {
    let answer = solve();
    println!("rozwiązanie dla zadania 005: {}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(solve(), 0);
    }
}
