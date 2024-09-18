use std::io::Read;

fn solve(frequencies: &[i64], key_sizes: &[i64]) -> i64 {
    let mut frequencies = frequencies.to_vec();
    frequencies.sort();
    frequencies.reverse();
    let mut keystrokes = vec![];
    for &s in key_sizes {
        (1..=s).for_each(|i| keystrokes.push(i));
    }
    keystrokes.sort();
    if keystrokes.len() < frequencies.len() {
        return -1;
    }
    frequencies.iter().zip(keystrokes.iter())
        .map(|(frequency, keystroke)| frequency * keystroke)
        .sum()
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let nf: usize = iter.next().unwrap().parse().unwrap();
    let nk: usize = iter.next().unwrap().parse().unwrap();
    let frequencies: Vec<i64> = (0..nf)
        .map(|_| iter.next().unwrap().parse().unwrap())
        .collect();
    let key_sizes: Vec<i64> = (0..nk)
        .map(|_| iter.next().unwrap().parse().unwrap())
        .collect();
    let result = solve(&frequencies, &key_sizes);
    println!("{}", result);
}
