// https://projecteuler.net/problem=4

// cargo run -p euler_004 --release

// use euler_utils::*;

fn solve() -> u64 {
    let mut max_pal = 0;

    for i in (100..999).rev() {
        for j in (100..999).rev() {
            let product = i * j;

            if product <= max_pal {
                break;
            }

            let s = product.to_string();

            if s.chars().eq(s.chars().rev()) {
                max_pal = product;
            }
        }
    }
    max_pal
}

fn main() {
    println!("{}", solve());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(solve(), 0);
    }
}
