// http://practice.geeksforgeeks.org/problems/leaders-in-an-array/0

use std::io;

fn leaders(a: Vec<i32>, n: usize) -> Vec<i32> {

    let mut m: i32 = 0;
    let mut r: Vec<i32> = Vec::new();

    for &i in a.iter().rev() {
        if i>=m {
            m = i;
            r.push(i);
        }
    }

    r.reverse();
    r

}

fn main() {

    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();

    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let a: Vec<i32> = a
        .trim()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();

    let r = leaders(a, n);
    println!("{}", r
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" "));

}
