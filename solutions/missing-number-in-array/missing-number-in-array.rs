use std::io;

fn main() {

    let mut nums = String::new();
    io::stdin().read_line(&mut nums).unwrap();
    let nums: Vec<i32> = nums
        .trim()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();

    let n = nums.len()+1;

    let mut a: Vec<bool> = vec![false; n];
    for &i in &nums {
        a[i as usize] = true;
    }

    for i in 0..n {
        if !a[i] {
            println!("{}", i as i32);
            return;
        }
    }

}
