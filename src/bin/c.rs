use std::cmp::Ordering;
use std::io::Read;

fn factorial(x: usize) -> usize {
    if x == 0 {
        1
    } else {
        x * factorial(x - 1)
    }
}

fn permutations(v: &[i64]) -> Vec<Vec<i64>> {
    fn rec(v: &mut [i64], i: usize) -> Vec<Vec<i64>> {
        if v.len() <= i {
            return vec![v.to_vec()];
        }
        let mut res = rec(v, i + 1);
        for j in i + 1..v.len() {
            v.swap(i, j);
            res.append(&mut rec(v, i + 1));
            v.swap(i, j);
        }
        res
    }

    rec(&mut v.to_vec(), 0)
}

fn solve(n: usize, a: &[i64], b: &[i64]) -> f64 {
    let mut a_wins = 0;
    for pa in permutations(a) {
        for pb in permutations(b) {
            let m = pa
                .iter()
                .zip(pb.iter())
                .map(|(x, y)| match x.cmp(y) {
                    Ordering::Greater => 1,
                    Ordering::Less => -1,
                    Ordering::Equal => 0,
                })
                .sum::<i64>();
            if m > 0 {
                a_wins += 1;
            }
        }
    }
    (a_wins as f64) / (factorial(n).pow(2) as f64)
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let a: Vec<i64> = (0..n)
        .map(|_| iter.next().unwrap().parse().unwrap())
        .collect();
    let b: Vec<i64> = (0..n)
        .map(|_| iter.next().unwrap().parse().unwrap())
        .collect();
    let result = solve(n, &a, &b);
    println!("{}", result);
}
