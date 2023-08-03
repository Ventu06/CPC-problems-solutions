// https://codeforces.com/problemset/problem/276/C

use std::io;
use std::iter;

fn solve(mut a: Vec<i64>, lr: Vec<(usize, usize)>) -> i64 {

    let mut v: Vec<i64> = vec![0;a.len()+1];

    for (l, r) in lr {
        v[l] += 1;
        v[r] -= 1;
    }

    v = v.iter().scan(0, |r, x| { *r = *r + x; Some(*r) }).collect();
    v.sort();
    a.sort();

    iter::zip(a.iter().rev(), v.iter().rev()).map(|(x, y)| x*y).sum()

}

fn main() {

    let mut nq = String::new();
    io::stdin().read_line(&mut nq).unwrap();
    let mut nq: Vec<usize> = nq
        .trim()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();
    let q = nq[1];

    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let mut a: Vec<i64> = a
        .trim()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();

    let mut lr: Vec<(usize, usize)> = Vec::new();
    for _ in 0..q {
        let mut liri = String::new();
        io::stdin().read_line(&mut liri).unwrap();
        let mut liri: Vec<usize> = liri
            .trim()
            .split(' ')
            .map(|x| x.parse().unwrap())
            .collect();
        lr.push((liri[0] - 1, liri[1]));
    }

    println!("{}", solve(a, lr));

}
