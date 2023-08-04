// https://practice.geeksforgeeks.org/problems/longest-common-subsequence-1587115620/1

use std::io;

fn solve(s1: &[u8], s2: &[u8]) -> i32 {

    let mut dp: Vec<Vec<i32>> = vec![vec![0; s2.len() + 1]; s1.len() + 1];

    for i in 0..s1.len() {
        for j in 0..s2.len() {
            if s1[i] == s2[j] {
                dp[i+1][j+1] = dp[i][j] + 1;
            } else {
                dp[i+1][j+1] = dp[i+1][j].max(dp[i][j+1]);
            }
        }
    }

    dp[s1.len()][s2.len()]

}

fn main() {

    let mut s1 = String::new();
    io::stdin().read_line(&mut s1).unwrap();
    let s1 = s1.trim().as_bytes();

    let mut s2 = String::new();
    io::stdin().read_line(&mut s2).unwrap();
    let s2 = s2.trim().as_bytes();

    println!("{}", solve(s1, s2));

}
