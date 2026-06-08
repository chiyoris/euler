// cargo run -p euler_${NUM} --release

// 1)
// fn factor(number: u64) -> Vec<u64> {
//     let mut v = vec![];
//     for x in 1..=((number as f64).sqrt() as u64) {
//         if number % x == 0 {
//             v.push(x);
//             if x != number / x {
//                 v.push(number / x);
//             }
//         }
//     }
//     v
// }

// fn check_prime(number: u64) -> bool {
//     if number < 2 {
//         return false;
//     }
//     for i in 2..=((number as f64).sqrt() as u64) {
//         if number % i == 0 && i != number {
//             return false;
//         }
//     }
//     return true;
// }

// fn solve(array: &[u64]) -> u64 {
//     *array.iter().filter(|&x| check_prime(*x)).max().unwrap()
// }
// fn main() {
//     let number = 13195;
//     // let number = 600851475143;
//     let factors = factor(number);
//     let answer = solve(&factors);
//     println!("rozwiązanie dla zadania 003: {}", answer);
// }

fn solve(mut n: u64) -> u64 {
    let mut largest = 0;

    while n % 2 == 0 {
        largest = 2;
        n /= 2;
    }

    let mut i = 3;
    while i * i <= n {
        while n % i == 0 {
            largest = i;
            n /= i;
        }
        i += 2;
    }

    if n > 1 {
        largest = n;
    }

    largest
}

fn main() {
    let answer = solve(600_851_475_143);
    println!("rozwiązanie dla zadania 003: {}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(solve(13195), 29);
    }
}
