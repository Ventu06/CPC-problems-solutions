// https://github.com/rossanoventurini/CompetitiveProgramming/blob/master/handson/handson03/problem_02/text.pdf
// https://github.com/rossanoventurini/CompetitiveProgramming/blob/master/handson/handson03/problem_02/Testset.zip

use std::io;

use problem_02::xmas_lights;

fn main() {
    let mut _n = String::new();
    io::stdin().read_line(&mut _n).unwrap();

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let s: Vec<char> = s.trim().chars().collect();

    println!("{}", xmas_lights(&s));
}
