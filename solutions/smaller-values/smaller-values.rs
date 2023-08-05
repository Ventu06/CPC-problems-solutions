// https://github.com/rossanoventurini/CompetitiveProgramming/blob/master/Exams/Text14012019.pdf
// https://github.com/rossanoventurini/CompetitiveProgramming/blob/master/Exams/TestSet14012019.zip

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

    fn query(&self, r: usize) -> i64 {
        self._query(r, 0, 1<<self.k, 1)
    }

    fn _query(&self, r: usize, lb: usize, rb: usize, p: usize) -> i64 {
        if r<=lb {
            0
        } else if rb<=r {
            self.d[p]
        } else {
            let _m = (lb+rb)/2;
            self._query(r,lb,_m,2*p) + self._query(r, _m, rb, 2*p+1)
        }
    }

    fn update(&mut self, j: usize) {
        let mut p = (1<<self.k)+j;
        while p>0 {
            self.d[p] += 1;
            p /=2 ;
        }
    }

}



fn solve(a: Vec<usize>, lrx: Vec<(usize, usize, usize)>) -> Vec<i64> {

    let mut ris: Vec<i64> = vec![0; lrx.len()];

    let mut lx: Vec<(usize, (usize, usize))> = lrx
        .iter()
        .map(|z| (z.0, z.2))
        .enumerate()
        .collect();
    let mut rx: Vec<(usize, (usize, usize))> = lrx
        .iter()
        .map(|z| (z.1, z.2))
        .enumerate()
        .collect();

    for (mut q, t) in [(lx, -1), (rx, 1)] {
        q.sort_by(|y, z| y.1.0.cmp(&z.1.0));
        let mut seg: SegTree = SegTree::new(a.len());
        let mut _j = 0;
        for (i, (j, x)) in q {
            for dj in _j..j {
                seg.update(a[dj]);
            }
            ris[i] += t*seg.query((x+1).min(a.len()));
            _j = j;
        }
    }

    ris

}

fn main() {

    let mut nm = String::new();
    io::stdin().read_line(&mut nm).unwrap();
    let nm: Vec<usize> = nm
        .trim()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();
    let m = nm[1];

    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let a: Vec<usize> = a
        .trim()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();

    let mut lrx: Vec<(usize, usize, usize)> = Vec::new();
    for _ in 0..m {
        let mut _lrx = String::new();
        io::stdin().read_line(&mut _lrx).unwrap();
        let _lrx: Vec<usize> = _lrx
            .trim()
            .split(' ')
            .map(|x| x.parse().unwrap())
            .collect();
        lrx.push((_lrx[0], _lrx[1]+1, _lrx[2]));
    }


    for r in solve(a, lrx) {
        println!("{}", r);
    }

}
