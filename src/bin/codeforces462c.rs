use std::io::Read;

fn solve(n: usize, a: &[i64]) -> i64 {
    let mut a = a.to_vec();
    a.sort();
    a.iter().enumerate().fold(0, |s, (i, x)| {
        s + x * (i as i64 + if i == n - 1 { 1 } else { 2 })
    })
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let a: Vec<i64> = (0..n)
        .map(|_| iter.next().unwrap().parse().unwrap())
        .collect();
    let result = solve(n, &a);
    println!("{}", result);
}
