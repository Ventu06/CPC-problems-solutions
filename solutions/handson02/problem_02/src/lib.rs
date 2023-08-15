use std::iter;

pub fn queries_of_operations(
    a: &[i32],
    os: &[(usize, usize, i32)],
    qs: &[(usize, usize)],
) -> Vec<i32> {
    let mut ad: Vec<i32> = vec![0]
        .iter()
        .chain(a.iter())
        .zip(a.iter())
        .map(|(&x, &y)| y - x)
        .chain(iter::once(0))
        .collect();

    let mut os_d: Vec<i32> = vec![0; os.len() + 1];
    for q in qs {
        os_d[q.0] += 1;
        os_d[q.1] -= 1;
    }

    let os_s = os_d[..os_d.len() - 1].iter().scan(0, |s, &x| {
        *s += x;
        Some(*s)
    });

    for (&(l, r, d), j) in os.iter().zip(os_s) {
        ad[l] += d * j;
        ad[r] -= d * j;
    }

    ad[..ad.len() - 1]
        .iter()
        .scan(0, |s, &x| {
            *s += x;
            Some(*s)
        })
        .collect()
}
