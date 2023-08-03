// https://codeforces.com/problemset/problem/652/D

use std::io;

struct SegTree {
    k: usize,
    d: Vec<usize>,
}

impl SegTree {

    fn new(v: Vec<usize>) -> Self {
        let mut seg = SegTree {k: 0, d: Vec::new()};
        while 1<<seg.k < v.len() {
            seg.k += 1;
        }
        seg.d.resize(1<<(seg.k+1),0);
        for i in 0 .. v.len() {
            seg.d[(1<<seg.k)+i] = v[i];
        }
        for j in (0 .. seg.k).rev() {
            for i in (1<<j) .. (1<<(j+1)) {
                seg.d[i] = seg.d[2*i] + seg.d[2*i+1];
            }
        }
        seg
    }

    fn get(&self, i: usize) -> usize {
        self.d[(1<<self.k)+i]
    }

    fn query(&self, r: usize) -> usize {
        self._query(0, r, 0, 1<<self.k, 1)
    }

    fn _query(&self, l: usize, r: usize, lb: usize, rb: usize, p: usize) -> usize {
        if l>=rb || r<=lb {
            0
        } else if l<=lb && rb<=r {
            self.d[p]
        } else {
            let _m = (lb+rb)/2;
            self._query(l,r,lb,_m,2*p) + self._query(l, r, _m, rb, 2*p+1)
        }
    }

    fn set(&mut self, j: usize) {
        let mut p = (1<<self.k)+j;
        self.d[p] = 1;
        p/=2;
        while p>0 {
            self.d[p] = self.d[2*p] + self.d[2*p+1];
            p /=2 ;
        }
    }

}


fn solve(lr: Vec<(i32, i32)>) -> Vec<usize> {

    let mut lre: Vec<(usize, &(i32, i32))> = lr.iter().enumerate().collect();
    lre.sort_by(|x, y| y.1.cmp(&x.1));

    let mut lre_t: Vec<(usize, i32)> = lre.iter().map(|x| x.1.1).enumerate().collect();
    lre_t.sort_by(|x, y| x.1.cmp(&y.1));

    let mut lre_f: Vec<(usize, usize)> = vec![(0, 0); lr.len()];
    for (i, &(j, _)) in lre_t.iter().enumerate() {
        lre_f[j].0 = lre[j].0;
        lre_f[j].1 = i;
    }

    let mut ris: Vec<usize> = vec![0; lr.len()];

    let mut seg: SegTree = SegTree::new(vec![0; lr.len()]);

    for (i, r) in lre_f {
        ris[i] = seg.query(r);
        seg.set(r);
    }

    ris

}

fn main() {

    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();

    let mut lr: Vec<(i32, i32)> = Vec::new();

    for _ in 0..n {
        let mut liri = String::new();
        io::stdin().read_line(&mut liri).unwrap();
        let liri: Vec<i32> = liri
            .trim()
            .split(' ')
            .map(|x| x.parse().unwrap())
            .collect();
        lr.push((liri[0], liri[1]));
    }

    for ris in solve(lr) {
        println!("{}", ris);
    }

}
