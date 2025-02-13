use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    #[allow(dead_code)]
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut r = 0;
        let mut h = BinaryHeap::from_iter(nums.into_iter().map(|x| Reverse(x as i64)));
        while let Some(m) = h.peek() {
            if m.0 >= k as i64 {
                break;
            }
            if let (Some(a), Some(b)) = (h.pop(), h.pop()) {
                h.push(Reverse(a.0.min(b.0) * 2 + a.0.max(b.0)));
            } else {
                break;
            }
            r += 1;
        }
        r
    }
}

#[allow(dead_code)]
struct Solution {}

fn main() {
    println!("nothing here");
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case1() {
        let actual = Solution::min_operations(vec![2, 11, 10, 1, 3], 10);
        assert_eq!(actual, 2);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::min_operations(vec![1, 1, 2, 4, 9], 20);
        assert_eq!(actual, 4);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::min_operations(vec![999999999, 999999999, 999999999], 1000000000);
        assert_eq!(actual, 2);
    }
}
