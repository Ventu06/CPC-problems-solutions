// https://leetcode.com/problems/trapping-rain-water/

use std::io;

fn trap_l<'a>(height: impl Iterator<Item = &'a i32>) -> i32 {

    let mut r = 0;
    let mut m = 0;
    for &h in height {
        m = m.max(h);
        r += m-h;
    }

    r

}

fn trap(height: Vec<i32>) -> i32 {

    let max_idx = height
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap()
        .0;

    let height_l = height[.. (max_idx+1)].iter();
    let height_r = height[max_idx ..].iter().rev();

    trap_l(height_l) + trap_l(height_r)
}

fn main() {

    let mut height = String::new();
    io::stdin().read_line(&mut height).unwrap();
    let height: Vec<i32> = height
        .trim()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();

    println!("{}", trap(height));

}
