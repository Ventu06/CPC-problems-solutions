// http://practice.geeksforgeeks.org/problems/leaders-in-an-array/0

use std::io;

fn main() {

    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();

    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let a: Vec<u32> = a
        .trim()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();

    let mut m: u32 = 0;
    let mut r: Vec<u32> = Vec::new();

    for &i in a.iter().rev() {
        if i>=m {
            m = i;
            r.push(i);
        }
    }

    println!("{}", r
        .iter()
        .rev()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" "))

}
