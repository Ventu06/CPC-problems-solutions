// https://codeforces.com/problemset/problem/466/C

use std::io;
use std::iter;

fn solve(a: Vec<i64>) -> usize {

    let s: i64 = a.iter().sum();

    if s%3 != 0 {
        0
    } else {

        let s:i64 = s/3;
        let v = a
            .iter()
            .scan((0,0), |r, x| {
                r.0 += x;
                if r.0 == s {
                    r.1 += 1
                }
                Some(r.1)
            });

        iter::zip(a.iter().skip(2), v.collect::<Vec<usize>>().iter())
            .rev()
            .fold((0,0), |r, (&x, &y)|
                  (r.0 + x, r.1 + (if r.0 + x == s { y } else { 0 }))
                )
            .1

    }

}

fn main() {

    let mut _n = String::new();
    io::stdin().read_line(&mut _n).unwrap();
    let _n: usize = _n.trim().parse().unwrap();

    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let mut a: Vec<i64> = a
        .trim()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();

    println!("{}", solve(a));

}
