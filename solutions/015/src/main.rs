// https://projecteuler.net/problem=015

// cargo run -p euler_015 --release  

// use euler_utils::*;


// fn fact(num: u128) -> u128 {
//     (1..=num).product()
// }
// fn solve(size: u128) -> u128 {
//     let n = size * 2;
//     let r = size;
//     fact(n)/(fact(n - r) * fact(r))
// }
// ^działa ale dla mniejszych liczb bez overflowow

fn solve(size: u128) -> u128{
    let n = size * 2;
    let mut result: u128 = 1;

    for i in 0..size{
        result = result * (n - i) / (i + 1);
    }

    result
}

fn main() {
    let answer = solve(20);
    println!("rozwiązanie dla zadania 015: {}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(solve(2), 6);
    }
}
