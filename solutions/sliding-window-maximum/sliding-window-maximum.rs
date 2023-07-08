// https://leetcode.com/problems/sliding-window-maximum/

use std::io;
use std::collections::VecDeque;

fn push_max(v: &mut VecDeque<(i32, usize)>, n: i32, i: usize) -> () {

    while let Some(a) = v.back() {
        if a.0 <= n {
            v.pop_back();
        } else {
            break;
        }
    }
    v.push_back((n, i));

}

fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {

    let k = k as usize;

    let mut v: VecDeque<(i32, usize)> = VecDeque::new();

    let mut r: Vec<i32> = Vec::new();

    for i in 0 .. k-1 {
        push_max(&mut v, nums[i], i);
    }

    for i in k-1 .. nums.len() {
        push_max(&mut v, nums[i], i);
        if v.front().unwrap().1 < i-(k-1) {
            v.pop_front();
        }
        r.push(v.front().unwrap().0);
    }

    r

}

fn main() {

    let mut nums = String::new();
    io::stdin().read_line(&mut nums).unwrap();
    let nums: Vec<i32> = nums
        .trim()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();

    let mut k = String::new();
    io::stdin().read_line(&mut k).unwrap();
    let k: i32 = k.trim().parse().unwrap();

    let r = max_sliding_window(nums, k);
    println!("{}", r
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" "));

}
