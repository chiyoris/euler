fn solve(limit: u32) -> u32 {
    let mut sum = 0;
    for x in 1..limit {
        if x % 3 == 0 || x % 5 == 0 {
            sum += x;
        }
    }
    sum
}

// fn solve(limit: u32) -> u32 {
//     (1..limit).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
// }

fn main() {
    let answer = solve(1000);
    println!("rozwiązanie dla zadania 001: {}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(solve(10), 23);
    }
}
