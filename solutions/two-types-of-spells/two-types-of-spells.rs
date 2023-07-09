//

use std::io;

fn solve(d: Vec<(bool, i64)>) -> Vec<i64> {

    //let mut l = 0;

    //for (b, p) in d {
        //if b {
            //if p>0 {
                //l += 1;
            //} else {
                //l -= 1;
            //}
        //}

    //}

}

fn main() {

    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();

    let mut d: Vec<(bool, i64)> = Vec::new();
    for _ in 0 .. n {
        let mut a = String::new();
        io::stdin().read_line(&mut a).unwrap();
        let mut a: (bool, i64) = (|xs: Vec<i64>| (xs[0]!=0, xs[1]))(a
            .trim()
            .split(' ')
            .map(|x| x.parse().unwrap())
            .collect());
        d.push(a);
    }

    for r in solve(d) {
        println!("{r}");
    }

}
