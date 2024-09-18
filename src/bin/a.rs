use std::collections::VecDeque;
use std::io::Read;

fn solve(n: usize, m: usize, cups: &[Vec<usize>; 3]) -> i64 {
    let mut q = VecDeque::new();
    let mut visited = vec![false; 3_usize.pow(n as u32)];
    let mut c = vec![0; n];
    for (i, v) in cups.iter().enumerate() {
        for &x in v {
            c[x - 1] = i;
        }
    }
    let mut pl = 0;
    for &i in &c {
        pl = pl * 3 + i;
    }
    visited[pl] = true;
    q.push_back((pl, 0));
    let finish0 = 0;
    let finish2 = 3_usize.pow(n as u32) - 1;
    while let Some((pl, cost)) = q.pop_front() {
        if pl == finish0 || pl == finish2 {
            return cost as i64;
        }
        if cost >= m {
            return -1;
        }
        let mut p = pl;
        let mut t0 = None;
        let mut t1 = None;
        let mut t2 = None;
        let mut b = 1;
        for _ in 0..n {
            let i = p % 3;
            match i {
                0 if t0.is_none() => {
                    t0 = Some(b);
                }
                1 if t1.is_none() => {
                    t1 = Some(b);
                }
                2 if t2.is_none() => {
                    t2 = Some(b);
                }
                _ => {}
            }
            b *= 3;
            p /= 3;
        }
        if let Some(x0) = t0 {
            if t1.map(|x1| x0 < x1).unwrap_or(t1.is_none()) {
                let pl = pl + x0;
                if !visited[pl] {
                    visited[pl] = true;
                    q.push_back((pl, cost + 1));
                }
            }
        }
        if let Some(x1) = t1 {
            if t0.map(|x0| x1 < x0).unwrap_or(t0.is_none()) {
                let pl = pl - x1;
                if !visited[pl] {
                    visited[pl] = true;
                    q.push_back((pl, cost + 1));
                }
            }
            if t2.map(|x2| x1 < x2).unwrap_or(t2.is_none()) {
                let pl = pl + x1;
                if !visited[pl] {
                    visited[pl] = true;
                    q.push_back((pl, cost + 1));
                }
            }
        }
        if let Some(x2) = t2 {
            if t1.map(|x1| x2 < x1).unwrap_or(t1.is_none()) {
                let pl = pl - x2;
                if !visited[pl] {
                    visited[pl] = true;
                    q.push_back((pl, cost + 1));
                }
            }
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
