use std::collections::{BinaryHeap, VecDeque};
use std::io::Read;
 
fn solve(n: usize, a: &[i64]) -> i64 {
    // [1]
    // [2, 2]
    // [3, 3, 2]
    // [4, 4, 3, 2] or [3, 3, 3, 3]
    //  +--+-- 3
    //  |  |
    //  |  +-- 3
    //  |
    //  +--+-- 3
    //     |
    //     +-- 3
    //
    //  +--+--+-- 4
    //  |  |  |
    //  |  |  +-- 4
    //  |  |
    //  |  +----- 3
    //  |
    //  +-------- 2
    let mut b = vec![];
    let mut h = BinaryHeap::from_iter(a.iter().enumerate().map(|(i, &x)| (x, i)));
    while h.len() > 1 {
        let (x, i) = h.pop().unwrap();
        let (y, j) = h.pop().unwrap();
        let k = n + b.len();
        b.push((i, j));
        h.push((x + y, k));
    }
    let mut c = vec![0; n];
    let mut q = VecDeque::new();
    q.push_back((n + b.len() - 1, 1));
    while let Some((i, s)) = q.pop_front() {
        if i < n {
            c[i] = s;
        } else {
            q.push_back((b[i - n].0, s + 1));
            q.push_back((b[i - n].1, s + 1));
        }
    }
    c.iter().zip(a.iter()).fold(0, |s, (x, y)| s + x * y)
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
