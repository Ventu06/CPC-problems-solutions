//

use std::io;

fn solve(a: Vec<i32>) -> i32 {

    0

}

fn main() {

    let mut _n = String::new();
    io::stdin().read_line(&mut _n).unwrap();
    let _n: usize = _n.trim().parse().unwrap();

    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let mut a: Vec<i32> = a
        .trim()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();

    println!("{}", solve(a));

}
