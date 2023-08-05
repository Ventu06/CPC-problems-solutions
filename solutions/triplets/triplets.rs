// https://github.com/rossanoventurini/CompetitiveProgramming/blob/master/Exams/Text14022018.pdf
// https://github.com/rossanoventurini/CompetitiveProgramming/blob/master/Exams/TestSet14022018.zip

use std::io;

struct SegTree {
    k: usize,
    d: Vec<i64>,
}

impl SegTree {

    fn new(len: usize) -> Self {
        let mut seg = SegTree {k: 0, d: Vec::new()};
        while 1<<seg.k < len {
            seg.k += 1;
        }
        seg.d.resize(1<<(seg.k+1),0);
        seg
    }

    fn query(&self, l: usize, r: usize) -> i64 {
        self._query(l, r, 0, 1<<self.k, 1)
    }

    fn _query(&self, l: usize, r: usize, lb: usize, rb: usize, p: usize) -> i64 {
        if l>=rb || r<=lb {
            0
        } else if l<=lb && rb<=r {
            self.d[p]
        } else {
            let _m = (lb+rb)/2;
            self._query(l,r,lb,_m,2*p) + self._query(l, r, _m, rb, 2*p+1)
        }
    }

    fn update(&mut self, j: usize, s: i64) {
        let mut p = (1<<self.k)+j;
        while p>0 {
            self.d[p] += s;
            p /=2 ;
        }
    }

}


fn solve(a: Vec<usize>) -> i64 {

    let mut seg_l: SegTree = SegTree::new(a.len());
    let mut seg_r: SegTree = SegTree::new(a.len());

    for &v in &a {
        seg_r.update(v, 1);
    }

    let mut r: i64 = 0;

    for &v in &a {
        seg_r.update(v, -1);
        r += seg_l.query(0, v)*seg_r.query(v+1, a.len());
        seg_l.update(v, 1);
    }

    r

}

fn main() {

    let mut _n = String::new();
    io::stdin().read_line(&mut _n).unwrap();
    let _n: usize = _n.trim().parse().unwrap();

    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let a: Vec<usize> = a
        .trim()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();

    println!("{}", solve(a));

}
