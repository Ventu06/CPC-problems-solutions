use binary_search_tree::BinarySearchTree;
use rand::Rng;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

pub fn brute_force(v: &Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let n = v.len();
    let mut maximums = Vec::with_capacity(n - k + 1);
    for i in 0..(n - k + 1) {
        let current_slice = &v[i..i + k];
        let max_value = *current_slice.iter().max().unwrap();
        maximums.push(max_value);
    }
    maximums
}

pub fn brute_force_idiomatic(v: &Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    v.windows(k).map(|w| *w.iter().max().unwrap()).collect()
}

pub fn heap(nums: &Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let mut s: BinaryHeap<(i32, usize)> = BinaryHeap::new();
    let mut r: Vec<i32> = Vec::new();

    for (i, &ni) in nums.iter().enumerate().take(k - 1) {
        s.push((ni, i));
    }

    for (i, &ni) in nums.iter().enumerate().skip(k - 1) {
        s.push((ni, i));
        while s.peek().unwrap().1 + k <= i {
            s.pop();
        }
        r.push(s.peek().unwrap().0);
    }

    r
}

pub fn bst(nums: &Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let mut s: BinarySearchTree<i32> = BinarySearchTree::new();
    let mut r: Vec<i32> = Vec::new();

    for &ni in nums.iter().take(k - 1) {
        s.insert(ni);
    }

    for i in k - 1..nums.len() {
        s.insert(nums[i]);
        r.push(*s.max().unwrap());
        s.remove(&nums[i + 1 - k]);
    }

    r
}

pub fn linear(nums: &Vec<i32>, k: i32) -> Vec<i32> {
    fn push_max(v: &mut VecDeque<(i32, usize)>, n: i32, i: usize) {
        while let Some(a) = v.back() {
            if a.0 <= n {
                v.pop_back();
            } else {
                break;
            }
        }
        v.push_back((n, i));
    }

    let k = k as usize;
    let mut v: VecDeque<(i32, usize)> = VecDeque::new();
    let mut r: Vec<i32> = Vec::new();

    for (i, &ni) in nums.iter().enumerate().take(k - 1) {
        push_max(&mut v, ni, i);
    }

    for (i, &ni) in nums.iter().enumerate().skip(k - 1) {
        push_max(&mut v, ni, i);
        if v.front().unwrap().1 < i - (k - 1) {
            v.pop_front();
        }
        r.push(v.front().unwrap().0);
    }

    r
}

pub fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut nums: Vec<i32> = Vec::with_capacity(n);
    let mut rng = rand::thread_rng();
    for _ in 0..n {
        nums.push(rng.gen_range(0..i32::MAX));
    }

    nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_idiomatic_version() {
        let k = 3;
        let v = gen_random_vector(100);

        let results = brute_force_idiomatic(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[ignore]
    #[test]
    fn test_heap_version() {
        let k = 3;
        let v = gen_random_vector(100);

        let results = heap(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[ignore]
    #[test]
    fn test_bst_version() {
        let k = 3;
        let v = gen_random_vector(100);

        let results = bst(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }

    #[ignore]
    #[test]
    fn test_linear_version() {
        let k = 3;
        let v = gen_random_vector(100);

        let results = linear(&v, k);
        let truth = brute_force(&v, k);

        assert_eq!(results, truth);
    }
}
