// https://leetcode.com/problems/maximum-subarray/

use std::io;
use std::cmp;

fn max_sub_array(nums: Vec<i32>) -> i32 {

    nums[1..].iter().fold(
        (nums[0], nums[0]),
        |(r,m), &x| {let rn = cmp::max(x,r+x); (rn, cmp::max(m,rn))}
        ).1

}

fn main() {

    let mut nums = String::new();
    io::stdin().read_line(&mut nums).unwrap();
    let nums: Vec<i32> = nums
        .trim()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();

    println!("{}", max_sub_array(nums));

}
