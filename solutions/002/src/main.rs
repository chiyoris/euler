// 1)
// fn fib(limit: u32) -> Vec<u32> {
//     let mut a = 1;
//     let mut b = 2;
//     let mut v = vec![a, b];
//     while a + b <= limit {
//         let temp = a + b;
//         a = b;
//         b = temp;
//         v.push(b);
//     }
//     v
// }

// 2)
// fn fib(limit: u32) -> Vec<u32> {
//     let mut v = vec![1, 2];
//     while v[v.len() - 1] + v[v.len() - 2] <= limit {
//         v.push(v[v.len() - 1] + v[v.len() - 2]);
//     }
//     v
// }
// fn solve(limit: u32) -> u32 {
//     fib(limit).iter().filter(|&x| x % 2 == 0).sum()
// }

// 4
// fn solve(limit: u32) -> u32 {
//     let (mut a, mut b, mut sum) = (1, 2, 0);
//     while a <= limit {
//         if a % 2 == 0 {
//             sum += a;
//         }
//         let next = a + b;
//         a = b;
//         b = next;
//     }
//     sum
// }

// 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
//    ^        ^          ^
// co trzeci wyraz jest parzysty.
// E(n) = 4 * E(n-1) + E(n-2)
fn solve(limit: u32) -> u32 {
    let (mut a, mut b) = (2, 8);
    let mut sum = 2;

    while b <= limit {
        sum += b;

        let next = 4 * b + a;
        a = b;
        b = next;
    }

    sum
}

fn main() {
    let answer = solve(4_000_000);
    println!("Rozwiązanie dla zadania 002: {}", answer);
}
