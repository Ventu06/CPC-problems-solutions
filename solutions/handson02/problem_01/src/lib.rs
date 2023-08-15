pub enum Query {
    Update { l: usize, r: usize, t: i32 },
    Max { l: usize, r: usize },
}

struct SegTree {
    k: usize,
    len: usize,
    d: Vec<i32>,
    e: Vec<i32>,
}

impl SegTree {
    fn new(v: &Vec<i32>) -> Self {
        let mut seg = SegTree {
            k: 0,
            len: v.len(),
            d: Vec::new(),
            e: Vec::new(),
        };
        while 1 << seg.k < seg.len {
            seg.k += 1;
        }
        seg.d.resize(1 << (seg.k + 1), 0);
        for (i, &vi) in v.iter().enumerate().take(seg.len) {
            seg.d[(1 << seg.k) + i] = vi;
        }
        for j in (0..seg.k).rev() {
            for i in (1 << j)..(1 << (j + 1)) {
                seg.d[i] = seg.d[2 * i].max(seg.d[2 * i + 1]);
            }
        }
        seg.e.resize(1 << (seg.k + 1), i32::MAX);
        seg
    }

    fn max(&mut self, l: usize, r: usize) -> i32 {
        self._max(l, r, 0, 1 << self.k, 1)
    }

    fn _max(&mut self, l: usize, r: usize, lb: usize, rb: usize, p: usize) -> i32 {
        if l >= rb || r <= lb {
            0
        } else if l <= lb && rb <= r {
            self.d[p].min(self.e[p])
        } else {
            self.e[2 * p] = self.e[2 * p].min(self.e[p]);
            self.e[2 * p + 1] = self.e[2 * p + 1].min(self.e[p]);
            self.d[p] = self.d[p].min(self.e[p]);
            self.e[p] = i32::MAX;
            let m = (lb + rb) / 2;
            self._max(l, r, lb, m, 2 * p)
                .max(self._max(l, r, m, rb, 2 * p + 1))
        }
    }

    fn update(&mut self, l: usize, r: usize, t: i32) {
        self._update(l, r, 0, 1 << self.k, 1, t);
    }

    fn _update(&mut self, l: usize, r: usize, lb: usize, rb: usize, p: usize, t: i32) {
        if l >= rb || r <= lb {
        } else if l <= lb && rb <= r {
            self.e[p] = self.e[p].min(t);
        } else {
            let m = (lb + rb) / 2;
            self._update(l, r, lb, m, 2 * p, t);
            self._update(l, r, m, rb, 2 * p + 1, t);
            self.d[p] =
                (self.d[2 * p].min(self.e[2 * p])).max(self.d[2 * p + 1].min(self.e[2 * p + 1]));
        }
    }
}

pub fn min_and_max(a: &Vec<i32>, qs: &Vec<Query>) -> Vec<i32> {
    let mut ris: Vec<i32> = Vec::new();

    let mut seg: SegTree = SegTree::new(a);
    for q in qs {
        match q {
            Query::Update { l, r, t } => {
                seg.update(*l, *r, *t);
            }
            Query::Max { l, r } => {
                ris.push(seg.max(*l, *r));
            }
        }
    }

    ris
}
