// https://codeforces.com/problemset/problem/52/C

use std::io;

struct SegTree {
    k: usize,
    len: usize,
    d: Vec<i64>,
    e: Vec<i64>,
}

impl SegTree {

    fn new(v: Vec<i64>) -> Self {
        let mut seg = SegTree {k: 0, len: v.len(), d: Vec::new(), e: Vec::new()};
        while 1<<seg.k < seg.len {
            seg.k += 1;
        }
        seg.d.resize(1<<(seg.k+1),0);
        for i in 0 .. seg.len {
            seg.d[(1<<seg.k)+i] = v[i];
        }
        for j in (0 .. seg.k).rev() {
            for i in (1<<j) .. (1<<(j+1)) {
                seg.d[i] = seg.d[2*i].min(seg.d[2*i+1]);
            }
        }
        seg.e.resize(1<<(seg.k+1),0);
        seg
    }

    fn query(&mut self, l: usize, r: usize) -> i64 {
        if l<r {
            self._query(l, r, 0, 1<<self.k, 1)
        } else {
            self._query(0, r, 0, 1<<self.k, 1).min(
                self._query(l, self.len, 0, 1<<self.k, 1)
                )
        }
    }

    fn _query(&mut self, l: usize, r: usize, lb: usize, rb: usize, p: usize) -> i64 {
        if l>=rb || r<=lb {
            i64::MAX
        } else if l<=lb && rb<=r {
            self.d[p] + self.e[p]
        } else {
            self.e[2*p] += self.e[p];
            self.e[2*p+1] += self.e[p];
            self.d[p] += self.e[p];
            self.e[p] = 0;
            let m = (lb+rb)/2;
            self._query(l,r,lb,m,2*p).min(self._query(l, r, m, rb, 2*p+1))
        }
    }

    fn update(&mut self, l: usize, r: usize, v: i64) {
        if l<r {
            self._update(l, r, 0, 1<<self.k, 1, v);
        } else {
            self._update(0, r, 0, 1<<self.k, 1, v);
            self._update(l, self.len, 0, 1<<self.k, 1, v);
        }
    }

    fn _update(&mut self, l: usize, r: usize, lb: usize, rb: usize, p: usize, v: i64) {
        if l>=rb || r<=lb {
        } else if l<=lb && rb<=r {
            self.e[p] += v;
        } else {
            let m = (lb+rb)/2;
            self._update(l,r,lb,m,2*p, v);
            self._update(l, r, m, rb, 2*p+1, v);
            self.d[p] = (self.d[2*p] + self.e[2*p]).min(self.d[2*p+1] + self.e[2*p+1]);
        }
    }

}



enum Query {
    Inc(usize, usize, i64),
    Rmq(usize, usize),
}


fn solve(a: Vec<i64>, qs: Vec<Query>) -> Vec<i64> {

    let mut seg = SegTree::new(a);

    let mut ris: Vec<i64> = Vec::new();

    for q in qs {
        match q {
            Query::Inc(l, r, v) => seg.update(l,r,v),
            Query::Rmq(l, r) => ris.push(seg.query(l,r)),
        }
    }

    ris

}

fn main() {

    let mut _n = String::new();
    io::stdin().read_line(&mut _n).unwrap();
    let _n: usize = _n.trim().parse().unwrap();

    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let a: Vec<i64> = a
        .trim()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();

    let mut m = String::new();
    io::stdin().read_line(&mut m).unwrap();
    let m: usize = m.trim().parse().unwrap();

    let mut q: Vec<Query> = Vec::new();
    for _ in 0..m {
        let mut lr_v = String::new();
        io::stdin().read_line(&mut lr_v).unwrap();
        let lr_v: Vec<&str> = lr_v.trim().split(' ').collect();
        if lr_v.len() == 2 {
            q.push(Query::Rmq(
                lr_v[0].parse::<usize>().unwrap(),
                lr_v[1].parse::<usize>().unwrap() + 1)
                );
        } else {
            q.push(Query::Inc(
                lr_v[0].parse::<usize>().unwrap(),
                lr_v[1].parse::<usize>().unwrap() + 1,
                lr_v[2].parse::<i64>().unwrap())
                );
        }
    }

    for r in solve(a, q) {
        println!("{}", r);
    }

}
