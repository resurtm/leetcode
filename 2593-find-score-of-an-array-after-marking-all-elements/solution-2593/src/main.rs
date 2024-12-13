use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

impl Solution {
    pub fn find_score(nums: Vec<i32>) -> i64 {
        let mut heap = BinaryHeap::new();
        nums.iter()
            .enumerate()
            .for_each(|(idx, it)| heap.push(Reverse((*it, idx))));

        let mut score = 0i64;
        let mut marked = HashSet::new();

        while let Some(Reverse(tup)) = heap.pop() {
            let (it, idx) = tup;
            if marked.contains(&idx) {
                continue;
            }
            score += it as i64;
            if idx > 0 {
                marked.insert(idx - 1);
            }
            marked.insert(idx);
            if idx < nums.len() - 1 {
                marked.insert(idx + 1);
            }
        }

        score
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case1() {
        assert_eq!(Solution::find_score(vec![2, 1, 3, 4, 5, 2]), 7);
    }

    #[test]
    fn test_case2() {
        assert_eq!(Solution::find_score(vec![2, 3, 5, 1, 3, 2]), 5);
    }
}

struct Solution;

fn main() {
    println!("Hello, world!");
}
