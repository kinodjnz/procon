use std::cmp::{max, min};
use std::io::Read;

fn solve(n: usize, xw: &[(i64, i64)]) -> usize {
    // x1 - x0 >= w1 + w0
    // x0 + w0 <= x1 - w1
    let mut dp = vec![i64::MAX; n + 1];
    dp[0] = i64::MIN;
    let mut k = 0;
    let mut xw = xw.to_vec();
    xw.sort_by_key(|z| z.0);
    for &(x, w) in &xw {
        let j = dp[0..=k].partition_point(|&p| p <= x - w);
        dp[j] = min(dp[j], x + w);
        k = max(k, j);
        println!("dp={:?}", &dp[0..=k]);
    }
    k
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let xw: Vec<(i64, i64)> = (0..n)
        .map(|_| {
            let x: i64 = iter.next().unwrap().parse().unwrap();
            let w: i64 = iter.next().unwrap().parse().unwrap();
            (x, w)
        })
        .collect();
    let result = solve(n, &xw);
    println!("{}", result);
}
