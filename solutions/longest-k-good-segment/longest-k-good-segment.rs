// https://codeforces.com/contest/616/problem/D

use std::io;
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;

fn solve(k: usize, a: Vec<i32>) -> (usize, usize) {

    let mut s = BTreeMap::new();
    let mut k: i32 = k as i32;

    let mut l: usize = 0;
    let mut v: Vec<(usize, usize)> = Vec::new();

    for r in 0..(a.len()) {
        match s.entry(a[r]) {
            Entry::Vacant(e) => {
                e.insert(1);
                k -= 1;
            },
            Entry::Occupied(mut e) => {
                e.insert(e.get() + 1);
            },
        }
        while k<0 {
            match s.entry(a[l]) {
                Entry::Vacant(_) => (),
                Entry::Occupied(mut e) => {
                    if *e.get() == 1 {
                        e.remove();
                        k += 1;
                    } else {
                        e.insert(e.get() - 1);
                    }
                },
            }
            l += 1;
        }
        v.push((r-l,l));
    }

    let ris = v.iter().max().unwrap();
    (ris.1, ris.1 + ris.0)

}

fn main() {

    let mut nk = String::new();
    io::stdin().read_line(&mut nk).unwrap();
    let nk: Vec<usize> = nk
        .trim()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();
    let k = nk[1];

    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let a: Vec<i32> = a
        .trim()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();

    let ris = solve(k, a);
    println!("{} {}", ris.0 + 1, ris.1 + 1);

}
