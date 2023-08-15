use std::iter;

pub fn xmas_lights(s: &[char]) -> u64 {
    fn foo<'a>(
        s: &'a [char],
        dp: impl Iterator<Item = u64> + 'a,
        k: u64,
        c: char,
    ) -> impl Iterator<Item = u64> + 'a {
        iter::once(k).chain(s.iter().zip(dp).scan(k, move |s, (&x, t)| {
            if x == c {
                *s += t;
            } else if x == 'X' {
                *s *= 3;
                *s += t;
            }
            Some(*s)
        }))
    }

    let _dp = vec![0; s.len()];
    let dp = _dp.iter().copied();
    let dp = foo(s, dp, 1, ' ');
    let dp = foo(s, dp, 0, 'R');
    let dp = foo(s, dp, 0, 'W');
    let dp = foo(s, dp, 0, 'G');

    dp.last().unwrap()
}
