// https://codeforces.com/problemset/problem/313/B

use std::io;
use std::iter;

fn solve(s: &str, lr: Vec<(usize, usize)>) -> Vec<usize> {

    let v: Vec<usize> = iter::once(0)
        .chain(iter::zip(s.chars(), s.chars().skip(1))
        .scan(0, |j, (x, y)| { *j += (x==y) as usize; Some(*j) }))
        .collect();

    lr.iter().map(|&(l, r)| v[r] - v[l]).collect()

}

fn main() {

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let s = s.trim();

    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();

    let mut lr: Vec<(usize, usize)> = Vec::new();

    for _ in 0..n {
        let mut liri = String::new();
        io::stdin().read_line(&mut liri).unwrap();
        let liri: Vec<usize> = liri
            .trim()
            .split(' ')
            .map(|x| x.parse().unwrap())
            .collect();
        lr.push((liri[0] - 1, liri[1] - 1));
    }

    for ris in solve(s, lr) {
        println!("{}", ris);
    }

}
