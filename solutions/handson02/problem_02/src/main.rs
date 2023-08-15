// https://github.com/rossanoventurini/CompetitiveProgramming/blob/master/handson/handson02/problem_02/text.pdf
// https://github.com/rossanoventurini/CompetitiveProgramming/blob/master/handson/handson02/problem_02/Testset.zip

use std::io;

use problem_02::queries_of_operations;

fn main() {
    let mut nmk = String::new();
    io::stdin().read_line(&mut nmk).unwrap();
    let nmk: Vec<usize> = nmk.trim().split(' ').map(|x| x.parse().unwrap()).collect();
    let m = nmk[1];
    let k = nmk[2];

    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let a: Vec<i32> = a.trim().split(' ').map(|x| x.parse().unwrap()).collect();

    let mut os: Vec<(usize, usize, i32)> = Vec::new();
    for _ in 0..m {
        let mut o = String::new();
        io::stdin().read_line(&mut o).unwrap();
        let o: Vec<&str> = o.trim().split(' ').collect();
        os.push((
            o[0].parse::<usize>().unwrap() - 1,
            o[1].parse::<usize>().unwrap(),
            o[2].parse::<i32>().unwrap(),
        ));
    }

    let mut qs: Vec<(usize, usize)> = Vec::new();
    for _ in 0..k {
        let mut q = String::new();
        io::stdin().read_line(&mut q).unwrap();
        let q: Vec<&str> = q.trim().split(' ').collect();
        qs.push((
            q[0].parse::<usize>().unwrap() - 1,
            q[1].parse::<usize>().unwrap(),
        ));
    }

    println!(
        "{}",
        queries_of_operations(&a, &os, &qs)
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
