// https://practice.geeksforgeeks.org/problems/minimum-number-of-jumps-1587115620/1

use std::io;

fn solve(a: Vec<usize>) -> i32 {

    let ai: Vec<usize> = a.iter().enumerate().map(|(i, x)| i+x).collect();

    let (mut l, mut r): (usize, usize) = (0, 1);
    let mut k: i32 = 0;
    while r < a.len() {
        match ai[l..r].iter().max() {
            Some(&m) => {
                (l, r) = (r, m+1);
                k += 1;
            },
            None => {
                k = -1;
                break;
            },
        }
    }

    k

}

fn main() {

    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let mut a: Vec<usize> = a
        .trim()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();

    println!("{}", solve(a));

}
