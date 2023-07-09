// https://leetcode.com/problems/next-greater-element-ii/

use std::io;

fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {

    let mut v: Vec<(usize, i32)> = Vec::new();
    let mut r: Vec<i32> = vec![-1; nums.len()];

    let max_idx = nums
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap()
        .0;

    let nums_ids = nums
        .iter()
        .enumerate()
        .skip(max_idx+1)
        .chain(nums
               .iter()
               .enumerate()
               .take(max_idx+1)
        );

    for (i, &m) in nums_ids {
        while let Some(&(j, n)) = v.last() {
            if m>n {
                r[j] = m;
                v.pop();
            } else {
                break;
            }
        }
        v.push((i,m));
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

    let r = next_greater_elements(nums);
    println!("{}", r
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" "));

}
