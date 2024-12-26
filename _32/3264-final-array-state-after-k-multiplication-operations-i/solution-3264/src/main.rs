use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn get_final_state(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        if nums.is_empty() || k == 0 {
            return nums;
        }
        let mut heap = BinaryHeap::new();
        nums.iter()
            .enumerate()
            .for_each(|x| heap.push(Reverse((*x.1, x.0))));
        let mut res = nums.clone();
        for _ in 0..k {
            if let Some(it) = heap.pop() {
                heap.push(Reverse((it.0 .0 * multiplier, it.0 .1)));
                res[it.0 .1] = it.0 .0 * multiplier;
            } else {
                panic!("invalid state");
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case1() {
        let actual = Solution::get_final_state(vec![2, 1, 3, 5, 6], 5, 2);
        assert_eq!(actual, vec![8, 4, 6, 5, 6]);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::get_final_state(vec![1, 2], 3, 4);
        assert_eq!(actual, vec![16, 8]);
    }
}

struct Solution;

fn main() {
    println!("Hello, world!");
}
