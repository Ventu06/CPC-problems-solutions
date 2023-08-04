// https://www.spoj.com/problems/KNAPSACK/

use std::io;

fn solve(s: usize, k: Vec<usize>, v: Vec<i32>) -> i32 {

    let mut dp: Vec<Vec<i32>> = vec![vec![0; s+1]; k.len()+1];

    for i in 0..k.len() {
        for j in 0..k[i] {
            dp[i+1][j] = dp[i][j];
        }
        for j in k[i]..(s+1) {
            dp[i+1][j] = dp[i][j].max(dp[i][j-k[i]] + v[i]);
        }
    }

    *dp[k.len()].iter().max().unwrap()

}

fn main() {

    let mut _sn = String::new();
    io::stdin().read_line(&mut _sn).unwrap();
    let _sn: Vec<&str> = _sn.trim().split(' ').collect();
    let s: usize = _sn[0].parse().unwrap();
    let n: usize = _sn[1].parse().unwrap();

    let mut k: Vec<usize> = Vec::new();
    let mut v: Vec<i32> = Vec::new();
    for _ in 0..n {
        let mut _kivi = String::new();
        io::stdin().read_line(&mut _kivi).unwrap();
        let _kivi: Vec<&str> = _kivi.trim().split(' ').collect();
        k.push(_kivi[0].parse().unwrap());
        v.push(_kivi[1].parse().unwrap());
    }

    println!("{}", solve(s, k, v));

}
