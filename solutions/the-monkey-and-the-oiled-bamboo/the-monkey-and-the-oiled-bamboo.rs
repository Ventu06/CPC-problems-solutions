// https://onlinejudge.org/index.php?option=onlinejudge&Itemid=8&page=show_problem&problem=3183

use std::io;
use std::iter::zip;

fn solve(r: Vec<i32>) -> i32 {

    let mut s: i32 = 0;

    for (h, k) in zip(r.iter(), r[1..].iter()) {
        if h-k>=s {
            s = (s+1).max(h-k);
        }
    }

    s

}

fn main() {

    let mut t = String::new();
    io::stdin().read_line(&mut t).unwrap();
    let t: usize = t.trim().parse().unwrap();

    for i in 0 .. t {

        let mut _n = String::new();
        io::stdin().read_line(&mut _n).unwrap();
        let _n: usize = _n.trim().parse().unwrap();

        let mut r = String::new();
        io::stdin().read_line(&mut r).unwrap();
        let mut r: Vec<i32> = r
            .trim()
            .split(' ')
            .map(|x| x.parse().unwrap())
            .collect();
        r.reverse();
        r.push(0);

        println!("Case {}: {}", i+1, solve(r));

    }

}
