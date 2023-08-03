// https://codeforces.com/problemset/problem/652/D

use std::io;
use std::collections::BTreeSet;

fn solve(lr: Vec<(i32, i32)>) -> Vec<usize> {

    let mut lre: Vec<(usize, &(i32, i32))> = lr.iter().enumerate().collect();
    lre.sort_by(|x, y| y.1.cmp(&x.1));

    let mut ris: Vec<usize> = vec![0; lr.len()];
    let mut s: BTreeSet<i32> = BTreeSet::new();

    for (i, &(_, r)) in lre {
        //O(n), but can be done easily in O(log(n)) if Rust supported it
        ris[i] = s.range(..r).count();
        s.insert(r);
    }

    ris

}

fn main() {

    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();

    let mut lr: Vec<(i32, i32)> = Vec::new();

    for _ in 0..n {
        let mut liri = String::new();
        io::stdin().read_line(&mut liri).unwrap();
        let liri: Vec<i32> = liri
            .trim()
            .split(' ')
            .map(|x| x.parse().unwrap())
            .collect();
        lr.push((liri[0], liri[1]));
    }

    for ris in solve(lr) {
        println!("{}", ris);
    }

}
