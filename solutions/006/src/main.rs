// https://projecteuler.net/problem=006

// cargo run -p euler_006 --release

// use euler_utils::*;
// fn solve(limit: u32) -> u32 {
//     let mut sum_of_squres: u32 = 0;
//     let mut square_of_sums: u32 = 0;
//     for x in 1..=limit {
//         sum_of_squres += x.pow(2);
//         square_of_sums += x;
//     }
//     square_of_sums.pow(2) - sum_of_squres
// }

fn solve(limit: u32) -> u32 {
    let sum = limit * (limit + 1) / 2;
    let sum_of_squares: u32 = (1..=limit).map(|x| x.pow(2)).sum();

    sum * sum - sum_of_squares
}

fn main() {
    let answer = solve(100);
    println!("rozwiązanie dla zadania 006: {}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(solve(10), 2640);
    }
}
