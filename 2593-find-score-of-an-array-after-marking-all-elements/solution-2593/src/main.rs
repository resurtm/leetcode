use std::collections::HashSet;

impl Solution {
    pub fn find_score(nums: Vec<i32>) -> i64 {
        let mut score = 0i64;
        let mut marked = HashSet::new();
        while marked.len() != nums.len() {
            let mut got_min = false;
            let mut min_idx = nums.len() - 1;
            let mut min = nums[min_idx];
            for (idx, it) in nums.iter().rev().enumerate() {
                if !marked.contains(&idx) && (!got_min || min >= *it) {
                    got_min = true;
                    min_idx = idx;
                    min = *it;
                }
            }
            score += min as i64;
            if min_idx > 0 {
                marked.insert(min_idx - 1);
            }
            marked.insert(min_idx);
            if min_idx < nums.len() - 1 {
                marked.insert(min_idx + 1);
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
