// https://practice.geeksforgeeks.org/problems/longest-increasing-subsequence-1587115620/1

use std::io;

fn solve(a: Vec<i32>) -> usize {

    let mut v: Vec<i32> = Vec::new();

    for x in a {
        let i = v.partition_point(|&z| z<x);
        if i < v.len() {
            v[i] = x;
        } else {
            v.push(x);
        }
    }

    v.len()

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
