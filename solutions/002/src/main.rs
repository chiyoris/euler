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

fn fib(limit: u32) -> Vec<u32> {
    let mut v = vec![1, 2];

    while v[v.len() - 1] + v[v.len() - 2] <= limit {
        v.push(v[v.len() - 1] + v[v.len() - 2]);
    }

    v
}

fn solve(limit: u32) -> u32 {
    fib(limit).iter().filter(|&x| x % 2 == 0).sum()
}

fn main() {
    let a = fib(4_000_000);
    for num in a {
        println!("{}", num);
    }

    let answer = solve(4_000_000);
    println!("Rozwiązanie dla zadania 002: {}", answer);
}
