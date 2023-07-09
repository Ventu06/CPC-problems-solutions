//

use std::io;
use std::collections::BTreeSet;
use std::iter::zip;


struct SegTree {
    k: usize,
    d: Vec<i64>,
}

impl SegTree {

    fn new(v: Vec<i64>) -> Self {
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
                seg.d[i] = seg.d[2*i].max(seg.d[2*i+1]);
            }
        }
        seg
    }

    fn get(&self, i: usize) -> i64 {
        self.d[(1<<self.k)+i]
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
            self._query(l,r,lb,_m,2*p).max(self._query(l, r, _m, rb, 2*p+1))
        }
    }

    fn update(&mut self, j: usize, s: i64) {
        let mut p = (1<<self.k)+j;
        self.d[p] += s;
        p/=2;
        while p>0 {
            self.d[p] = self.d[2*p].max(self.d[2*p+1]);
            p /=2 ;
        }
    }

}

fn solve(x: Vec<i64>, t: Vec<i64>, p: Vec<i64>, b: Vec<i64>) -> Vec<(usize, i64)> {

    let n: usize = x.len();

    let t: Vec<i64> = zip(&x,&t).map(|(x,y)| x+y).collect();

    let mut a: Vec<(i64, i64, usize)> = zip(x,t).enumerate().map(|(z,(x,y))| (x,y,z)).collect();
    a.sort();

    let lf: Vec<i64> = a.iter().map(|x| x.0).collect();
    let rf: Vec<i64> = a.iter().map(|x| x.1).collect();

    let mut seg: SegTree = SegTree::new(rf);
    let mut q: BTreeSet<(i64, i64, usize)> = BTreeSet::new();
    let mut ris: Vec<(usize, i64)> = vec![(0,0); n];

    for (i, pe, be) in zip(p,b).enumerate().map(|(x,(y,z))| (x,y,z)) {

        q.insert((pe,be,i));

        let mut l: usize = 0;
        let mut r: usize = lf.partition_point(|&x| x<=pe);

        if seg.query(l,r) < pe {
            continue;
        }

        while r-l > 1 {
            let m = (l+r)/2;
            if seg.query(l,m) < pe {
                l = m;
            }else{
                r = m;
            }
        }

        let lp = lf[l];
        let mut rp = seg.get(l);

        let mut q_rem: Vec<(i64, i64, usize)> = Vec::new();
        let it_ran = q.range((lp,0,0) ..);
        for &it in it_ran {
            let (peu, beu, _) = it;
            if rp >= peu {
                ris[a[l].2].0 += 1;
                seg.update(l, beu);
                rp += beu;
                q_rem.push(it);
            } else {
                break;
            }
        }
        for it in q_rem {
            q.remove(&it);
        }

    }

    for i in 0 .. n {
        ris[a[i].2].1 = seg.get(i)-lf[i];
    }

    ris

}

fn main() {

    let mut nm = String::new();
    io::stdin().read_line(&mut nm).unwrap();
    let (n, m): (usize, usize) = (|xs: Vec<usize>| (xs[0], xs[1]))(nm
        .trim()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect()
        );

    let mut x: Vec<i64> = Vec::new();
    let mut t: Vec<i64> = Vec::new();
    for _ in 0 .. n {
        let mut xete = String::new();
        io::stdin().read_line(&mut xete).unwrap();
        let (xe, te): (i64, i64) = (|xs: Vec<i64>| (xs[0], xs[1]))(xete
            .trim()
            .split(' ')
            .map(|x| x.parse().unwrap())
            .collect()
            );
        x.push(xe);
        t.push(te);
    }

    let mut p: Vec<i64> = Vec::new();
    let mut b: Vec<i64> = Vec::new();
    for _ in 0 .. m {
        let mut pebe = String::new();
        io::stdin().read_line(&mut pebe).unwrap();
        let (pe, be): (i64, i64) = (|xs: Vec<i64>| (xs[0], xs[1]))(pebe
            .trim()
            .split(' ')
            .map(|x| x.parse().unwrap())
            .collect()
            );
        p.push(pe);
        b.push(be);
    }

    let r = solve(x, t, p, b);

    for (c, l) in r {
        println!("{c} {l}");
    }

}
