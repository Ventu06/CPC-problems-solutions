use std::io;
use std::cmp;

fn main() {

    let mut nums = String::new();
    io::stdin().read_line(&mut nums).unwrap();
    let nums: Vec<i32> = nums
        .trim()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();

    let mut r: i32 = nums[0];
    let mut m: i32 = nums[0];

    for &i in &nums[1..] {
        r = cmp::max(i,r+i);
        m = cmp::max(m,r);
    }

    println!("{m}");

}
