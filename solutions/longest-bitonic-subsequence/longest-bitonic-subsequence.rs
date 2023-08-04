// https://practice.geeksforgeeks.org/problems/longest-bitonic-subsequence0824/1

use std::io;
use std::iter;

fn lis<'a>(a: impl Iterator<Item=&'a i32>) -> Vec<usize> {

    let mut ris: Vec<usize> = Vec::new();

    let mut p: Vec<i32> = Vec::new();
    for &v in a {
        let i = p.partition_point(|&x| x<v);
        if i < p.len() {
            p[i] = v;
        } else {
            p.push(v);
        }
        ris.push(p.len());
    }

    ris

}

fn solve(a: Vec<i32>) -> usize {

    iter::zip(lis(a.iter()).iter(), lis(a.iter().rev()).iter().rev())
        .map(|(&x, &y)| x+y)
        .max()
        .unwrap()
        - 1
}

fn main() {

    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let a: Vec<i32> = a
        .trim()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();

    println!("{}", solve(a));

}
