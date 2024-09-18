use std::collections::VecDeque;
use std::io::Read;

fn solve(n: usize, m: usize, cups: &[Vec<usize>; 3]) -> i64 {
    let mut q = VecDeque::new();
    let cups: Vec<Vec<usize>> = cups
        .iter()
        .map(|v| {
            let mut v = v.clone();
            v.insert(0, 0);
            v
        })
        .collect();
    q.push_back((cups.clone(), 0, -1));
    let finished: Vec<usize> = (0..=n).collect();
    while let Some((cups, cost, prev)) = q.pop_front() {
        if cups[0] == finished || cups[2] == finished {
            return cost as i64;
        }
        if cost >= m {
            return -1;
        }
        let a = *cups[0].last().unwrap();
        let b = *cups[1].last().unwrap();
        let c = *cups[2].last().unwrap();
        if a > b && prev != 1 {
            let mut next = cups.clone();
            next[0].pop();
            next[1].push(a);
            q.push_back((next, cost + 1, 0));
        }
        if a < b && prev != 0 {
            let mut next = cups.clone();
            next[1].pop();
            next[0].push(b);
            q.push_back((next, cost + 1, 1));
        }
        if b > c && prev != 3 {
            let mut next = cups.clone();
            next[1].pop();
            next[2].push(b);
            q.push_back((next, cost + 1, 2));
        }
        if b < c && prev != 2 {
            let mut next = cups.clone();
            next[2].pop();
            next[1].push(c);
            q.push_back((next, cost + 1, 3));
        }
    }
    -1
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    loop {
        let n: usize = iter.next().unwrap().parse().unwrap();
        let m: usize = iter.next().unwrap().parse().unwrap();
        if n == 0 && m == 0 {
            break;
        }
        let c: Vec<Vec<usize>> = (0..3)
            .map(|_| {
                let c = iter.next().unwrap().parse().unwrap();
                (0..c)
                    .map(|_| iter.next().unwrap().parse().unwrap())
                    .collect()
            })
            .collect();
        let cups = [c[0].clone(), c[1].clone(), c[2].clone()];
        let result = solve(n, m, &cups);
        println!("{}", result);
    }
}
