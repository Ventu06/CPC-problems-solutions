// https://github.com/rossanoventurini/CompetitiveProgramming/blob/master/handson/handson03/problem_01/text.pdf
// https://github.com/rossanoventurini/CompetitiveProgramming/blob/master/handson/handson03/problem_01/Testset.zip

use std::io;

use problem_01::holiday_planning;

fn main() {
    let mut nd = String::new();
    io::stdin().read_line(&mut nd).unwrap();
    let nd: Vec<usize> = nd.trim().split(' ').map(|x| x.parse().unwrap()).collect();
    let n = nd[0];

    let mut attr: Vec<Vec<u32>> = Vec::new();

    for _ in 0..n {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let s: Vec<u32> = s.trim().split(' ').map(|x| x.parse().unwrap()).collect();
        attr.push(s);
    }

    println!("{}", holiday_planning(&attr));
}
