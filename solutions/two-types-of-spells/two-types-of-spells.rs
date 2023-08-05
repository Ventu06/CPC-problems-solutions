// https://codeforces.com/contest/1398/problem/E

use std::io;
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;


const FIRE: bool = false;
const LIGHT: bool = true;


fn get_high(s: &BTreeMap<i64, (usize, usize)>) -> (i64, bool) {
    match s.last_key_value() {
        None => (0, false),
        Some((&p, &(f_n, _))) => {
            if f_n > 0 {
                (p, FIRE)
            } else {
                (p, LIGHT)
            }
        },
    }
}


fn get_low(s: &BTreeMap<i64, (usize, usize)>) -> (i64, bool) {
     match s.first_key_value() {
        None => (0, false),
        Some((&p, &(_, l_n))) => {
            if l_n > 0 {
                (p, LIGHT)
            } else {
                (p, FIRE)
            }
        },
     }
}

fn add(s: &mut BTreeMap<i64, (usize, usize)>, x: (i64, bool), light_cnt: &mut usize) {
    if x.1 == FIRE {
        s.entry(x.0).and_modify(|z| z.0 += 1).or_insert((1,0));
    } else {
        s.entry(x.0).and_modify(|z| z.1 += 1).or_insert((0,1));
        *light_cnt += 1;
    }
}

fn del(s: &mut BTreeMap<i64, (usize, usize)>, x: (i64, bool), light_cnt: &mut usize) {
    match s.entry(x.0) {
        Entry::Occupied(mut e) => {
            let z = e.get_mut();
            if x.1 == FIRE {
                z.0 -= 1;
            } else {
                z.1 -= 1;
                *light_cnt -= 1;
            }
            if *z == (0,0) {
                e.remove();
            }
        }
        _ => {},
    }
}


fn solve(d: Vec<(i64, bool)>) -> Vec<i64> {

    let mut ris: Vec<i64> = Vec::new();

    let mut high: BTreeMap<i64, (usize, usize)> = BTreeMap::new();
    let mut low: BTreeMap<i64, (usize, usize)> = BTreeMap::new();
    let mut high_light_n: usize = 0;
    let mut low_light_n: usize = 0;

    let mut s = 0;
    for (p, t) in d {
        s += p;
        if t==LIGHT {
            if p>0 {
                if low.len() == 0 || {
                    let (_p, _t) = get_high(&mut low);
                    _p < p || _p == p && _t==LIGHT
                    } {
                    add(&mut high, (p, t), &mut high_light_n);
                    s += p;
                } else {
                    add(&mut low, (p, t), &mut low_light_n);
                    let z = get_high(&mut low);
                    del(&mut low, z, &mut low_light_n);
                    add(&mut high, z, &mut high_light_n);
                    s += z.0;
                }
            } else {
                let p = -p;
                if high.len() != 0 && {
                    let (_p, _t) = get_low(&mut high);
                    _p < p || _p == p && _t==LIGHT
                    } {
                    del(&mut high, (p, t), &mut high_light_n);
                    s -= p;
                } else {
                    del(&mut low, (p, t), &mut low_light_n);
                    let z = get_low(&mut high);
                    add(&mut low, z, &mut low_light_n);
                    del(&mut high, z, &mut high_light_n);
                    s -= z.0;
                }
            }
        } else {
            if p>0 {
                if high.len() == 0 || {
                    let (_p, _t) = get_low(&mut high);
                    _p > p || _p == p && _t==FIRE
                    } {
                    add(&mut low, (p, t), &mut low_light_n);
                } else {
                    add(&mut high, (p, t), &mut high_light_n);
                    let z = get_low(&mut high);
                    del(&mut high, z, &mut high_light_n);
                    add(&mut low, z, &mut low_light_n);
                    s += p - z.0;
                }
            } else {
                let p = -p;
                if low.len() != 0 && {
                    let (_p, _t) = get_high(&mut low);
                    _p > p || _p == p && _t==FIRE
                    } {
                    del(&mut low, (p, t), &mut low_light_n);
                } else {
                    del(&mut high, (p, t), &mut high_light_n);
                    let z = get_high(&mut low);
                    add(&mut high, z, &mut high_light_n);
                    del(&mut low, z, &mut low_light_n);
                    s -= p - z.0;
                }
            }
        }
        if low_light_n > 0 || high_light_n == 0 {
            ris.push(s);
        } else if low.len() > 0 {
            ris.push(s - get_low(&high).0 + get_high(&low).0);
        } else {
            ris.push(s - get_low(&high).0);
        }
    }

    ris

}

fn main() {

    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();

    let mut d: Vec<(i64, bool)> = Vec::new();
    for _ in 0 .. n {
        let mut a = String::new();
        io::stdin().read_line(&mut a).unwrap();
        let a: (i64, bool) = (|xs: Vec<i64>| (xs[1], xs[0]!=0))(a
            .trim()
            .split(' ')
            .map(|x| x.parse().unwrap())
            .collect());
        d.push(a);
    }

    for r in solve(d) {
        println!("{r}");
    }

}
