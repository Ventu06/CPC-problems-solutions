// https://practice.geeksforgeeks.org/problems/subset-sum-problem2014/1

use std::io;

fn solve(a: Vec<usize>) -> bool {

    let s: usize = a.iter().sum();

    if s%2 != 0 {
        false
    } else {
        let s = s/2;
        let mut dp: Vec<Vec<bool>> = vec![vec![false; s+1]; a.len()+1];
        dp[0][0] = true;
        for i in 0..a.len() {
            for j in 0..a[i] {
                dp[i+1][j] = dp[i][j];
            }
            for j in a[i]..(s+1) {
                dp[i+1][j] = dp[i][j] || dp[i][j-a[i]];
            }
        }
        dp[a.len()][s]
    }

}

fn main() {

    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let a: Vec<usize> = a
        .trim()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();

    if solve(a) {
        println!("YES");
    } else {
        println!("NO");
    }

}
