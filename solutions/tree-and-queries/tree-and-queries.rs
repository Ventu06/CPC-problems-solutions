// https://codeforces.com/contest/375/problem/D

use std::io;

fn make_tree(ab: Vec<(usize, usize)>) -> (Vec<usize>, Vec<(usize, usize)>) {

    let n = ab.len() + 1;

    let mut ris1: Vec<usize> = Vec::new();
    let mut ris2: Vec<(usize, usize)> = vec![(0, 0); n];

    let mut e: Vec<Vec<usize>> = vec![vec![]; n];
    for (a, b) in ab {
        e[a].push(b);
        e[b].push(a);
    }

    let mut st: Vec<usize> = Vec::new();
    let mut vis: Vec<bool> = vec![false; n];

    st.push(0);
    st.push(0);

    while st.len() >= 2{
        let p = st[st.len()-1];
        if vis[p] {
            ris2[p].1 = ris1.len();
            st.pop();
        } else {
            vis[p] = true;
            ris2[p].0 = ris1.len();
            ris1.push(p);
            let q = st[st.len()-2];
            st.extend(e[p].iter().filter(|x| **x != q));
        }
    }

    (ris1, ris2)

}

fn solve(c: Vec<usize>, ab: Vec<(usize, usize)>, vk: Vec<(usize, usize)>) -> Vec<usize> {

    let sn = 320;
    let sc = 100010;

    let (a, lr) = make_tree(ab);

    let mut ris: Vec<usize> = vec![0; vk.len()];

    let c: Vec<usize> = a.iter().map(|&p| c[p]).collect();

    let ilrk = vk.iter().map(|&(p, k)| ((lr[p]), k)).enumerate();

    let mut bs: Vec<Vec<(usize, ((usize, usize), usize))>> = vec![vec![]; sn];
    for x in ilrk {
        bs[x.1.0.0/sn].push(x);
    }

    let mut cnum = vec![0; sc];
    let mut suff = vec![0; sc];

    for mut b in bs {
        cnum.iter_mut().for_each(|x| *x = 0);
        suff.iter_mut().for_each(|x| *x = 0);
        b.sort_by(|x, y| x.1.0.1.cmp(&y.1.0.1));
        let mut _l: usize = 0;
        let mut _r: usize = 0;
        for (i, ((l, r), k)) in b {
            for &v in &c[_r..r] {
                cnum[v] += 1;
                suff[cnum[v]] += 1;
            }
            if l < _l {
                for &v in &c[l.._l] {
                    cnum[v] += 1;
                    suff[cnum[v]] += 1;
                }
            } else {
                for &v in &c[_l..l] {
                    suff[cnum[v]] -= 1;
                    cnum[v] -= 1;
                }
            }
            ris[i] = suff[k];
            _l = l;
            _r = r;
        }
    }

    ris

}

fn main() {

    let mut _s = String::new();
    io::stdin().read_line(&mut _s).unwrap();
    let _s: Vec<usize> = _s
        .trim()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();
    let n: usize = _s[0];
    let m: usize = _s[1];

    let mut _s = String::new();
    io::stdin().read_line(&mut _s).unwrap();
    let c: Vec<usize> = _s
        .trim()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();

    let mut ab: Vec<(usize, usize)> = Vec::new();
    for _ in 0..n-1 {
         let mut _s = String::new();
        io::stdin().read_line(&mut _s).unwrap();
        let _s: Vec<usize> = _s
            .trim()
            .split(' ')
            .map(|x| x.parse().unwrap())
            .collect();
        ab.push((_s[0] - 1, _s[1] - 1));
    }


    let mut vk: Vec<(usize, usize)> = Vec::new();
    for _ in 0..m {
        let mut _s = String::new();
        io::stdin().read_line(&mut _s).unwrap();
        let _s: Vec<usize> = _s
            .trim()
            .split(' ')
            .map(|x| x.parse().unwrap())
            .collect();
        vk.push((_s[0] - 1, _s[1]));
    }

    for r in solve(c, ab, vk) {
        println!("{}", r);
    }

}
