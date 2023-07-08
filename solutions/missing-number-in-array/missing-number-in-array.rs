use std::io;

fn missing_number(nums: Vec<i32>) -> i32 {

    let n = nums.len()+1;

    let mut a: Vec<bool> = vec![false; n];
    for &i in &nums {
        a[i as usize] = true;
    }

    a.iter().position(|&x| x==false).unwrap() as i32

}

fn main() {

    let mut nums = String::new();
    io::stdin().read_line(&mut nums).unwrap();
    let nums: Vec<i32> = nums
        .trim()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();

    println!("{}", missing_number(nums));

}
