// https://github.com/rossanoventurini/CompetitiveProgramming/blob/master/handson/handson02/problem_01/text.pdf
// https://github.com/rossanoventurini/CompetitiveProgramming/blob/master/handson/handson02/problem_01/Testset.zip

use std::io;

use problem_01::{min_and_max, Query};

fn main() {
    let mut nm = String::new();
    io::stdin().read_line(&mut nm).unwrap();
    let nm: Vec<usize> = nm.trim().split(' ').map(|x| x.parse().unwrap()).collect();
    let m = nm[1];

    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let a: Vec<i32> = a.trim().split(' ').map(|x| x.parse().unwrap()).collect();

    let mut qs: Vec<Query> = Vec::new();

    for _ in 0..m {
        let mut q = String::new();
        io::stdin().read_line(&mut q).unwrap();
        let q: Vec<&str> = q.trim().split(' ').collect();
        if q[0] == "0" {
            qs.push(Query::Update {
                l: q[1].parse::<usize>().unwrap() - 1,
                r: q[2].parse::<usize>().unwrap(),
                t: q[3].parse::<i32>().unwrap(),
            });
        } else {
            qs.push(Query::Max {
                l: q[1].parse::<usize>().unwrap() - 1,
                r: q[2].parse::<usize>().unwrap(),
            });
        }
    }

    for r in min_and_max(&a, &qs) {
        println!("{}", r);
    }
}
