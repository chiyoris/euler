// https://projecteuler.net/problem=009

// cargo run -p euler_009 --release

// use euler_utils::*;

fn solve(num: i32) -> (i32, i32, i32) {
    for a in 1..num {
        for b in a + 1..num {
            let c2 = a * a + b * b;

            let c = ((c2) as f64).sqrt() as i32;

            if c * c == c2 && a + b + c == 1000 {
                return (a, b, c);
            }
        }
    }
    (0, 0, 0)
}

fn main() {
    let (a, b, c) = solve(1000);
    println!("rozwiązanie dla zadania 009: {} {} {}", a, b, c);
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_example() {
//         assert_eq!(solve(0), 0);
//     }
// }
