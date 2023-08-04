// https://codeforces.com/contest/86/problem/D

use std::io;
use std::iter;

fn solve(a: Vec<i64>, lr: Vec<(usize, usize)>) -> Vec<i64> {

    let mut ae: Vec<(usize, i64)> = a
        .iter()
        .enumerate()
        .map(|(i, &x)| (i, x))
        .collect();
    ae.sort_by(|x, y| x.1.cmp(&y.1));
    let mut am: Vec<i64> = Vec::new();
    let mut an: Vec<usize> = vec![0; a.len()];

    am.push(ae[0].1);
    an[0] = 0;
    for i in 1..a.len() {
        if ae[i].1 != ae[i-1].1 {
            am.push(ae[i].1);
        }
        an[ae[i].0] = am.len()-1;
    }

    let n = 500;
    let k = am.len();

    let mut ris = vec![0; lr.len()];

    let mut bs: Vec<Vec<(usize, (usize, usize))>> = vec![vec![]; n];

    for (i, &x) in lr.iter().enumerate() {
        bs[x.0/n].push((i, x));
    }

    for (ib, b) in bs.iter_mut().enumerate() {
        b.push((0, (ib*n, ib*n)));
        b.sort_by(|x, y| x.1.1.cmp(&y.1.1));

        let mut s = 0;
        let mut ac: Vec<i64> = vec![0; k];
        for (&(_, (l_, r_)), &(i, (l, r))) in iter::zip(b.iter(), b.iter().skip(1)) {
            for &v in &an[r_..r] {
                s += am[v]*(2*ac[v]+1);
                ac[v] += 1;
            }
            if l > l_ {
                for &v in &an[l_..l] {
                    ac[v] -= 1;
                    s -= am[v]*(2*ac[v]+1);
                }
            } else {
                for &v in &an[l..l_] {
                    s += am[v]*(2*ac[v]+1);
                    ac[v] += 1;
                }
            }
            ris[i] = s;
        }

    }

    ris

}

fn main() {

    let mut nt = String::new();
    io::stdin().read_line(&mut nt).unwrap();
    let nt: Vec<usize> = nt
        .trim()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();
    let t = nt[1];

    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let a: Vec<i64> = a
        .trim()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();

    let mut lr: Vec<(usize, usize)> = Vec::new();

    for _ in 0..t {
        let mut liri = String::new();
        io::stdin().read_line(&mut liri).unwrap();
        let liri: Vec<usize> = liri
            .trim()
            .split(' ')
            .map(|x| x.parse().unwrap())
            .collect();
        lr.push((liri[0] - 1, liri[1]));
    }

    for ris in solve(a, lr) {
        println!("{}", ris);
    }

}
