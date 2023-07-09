// http://codeforces.com/problemset/problem/37/A?locale=en

use std::io;


fn main() {

    let mut _n = String::new();
    io::stdin().read_line(&mut _n).unwrap();
    let _n: usize = _n.trim().parse().unwrap();

    let mut l = String::new();
    io::stdin().read_line(&mut l).unwrap();
    let mut l: Vec<i32> = l
        .trim()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();

    l.sort();
    let mut r_n = 0;
    let mut r_h = 0;
    let mut m_h = 1;

    for i in 0 .. l.len() {
        if i+1<l.len() && l[i]==l[i+1] {
            m_h += 1;
        }else{
            r_h = r_h.max(m_h);
            m_h = 1;
            r_n += 1;
        }
    }

    println!("{r_h} {r_n}");

}
