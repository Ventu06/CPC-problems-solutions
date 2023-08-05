// https://www.spoj.com/problems/INVCNT/

use std::io;


fn solve(a: &mut [i32]) -> i64 {

    if a.len() <= 1 {
        0
    } else {
        let nl = a.len()/2;
        let nr = a.len()-nl;
        let (al, ar) = a.split_at_mut(nl);
        let mut r = solve(al) + solve(ar);
        let mut at: Vec<i32> = Vec::new();
        let mut jl: usize = 0;
        let mut jr: usize = 0;
        while jl+jr<nl+nr {
            if jl < nl && (jr == nr || al[jl] < ar[jr]){
                at.push(al[jl]);
                jl += 1;
                r += jr as i64;
            } else {
                at.push(ar[jr]);
                jr += 1;
            }
        }
        a.clone_from_slice(&at);
        r
    }

}

fn main() {

    let mut t = String::new();
    io::stdin().read_line(&mut t).unwrap();
    let t: usize = t.trim().parse().unwrap();

    for _ in 0..t {

        let mut _s = String::new();
        io::stdin().read_line(&mut _s).unwrap();

        let mut n = String::new();
        io::stdin().read_line(&mut n).unwrap();
        let n: usize = n.trim().parse().unwrap();

        let mut a: Vec<i32> = Vec::new();
        for _ in 0..n {
            let mut x = String::new();
            io::stdin().read_line(&mut x).unwrap();
            let x: i32 = x.trim().parse().unwrap();
            a.push(x);
        }

        println!("{}", solve(&mut a));

    }

}
