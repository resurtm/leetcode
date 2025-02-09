use std::collections::HashMap;

impl Solution {
    #[allow(dead_code)]
    pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
        let (mut res, len) = (0, nums.len());
        let count = (len * (len - 1) / 2) as i64;
        let mut cache = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            let val = i as i32 - num;
            res += *cache.get(&val).unwrap_or(&0);
            cache.insert(val, *cache.get(&val).unwrap_or(&0) + 1);
        }
        count - res
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
        let actual = Solution::count_bad_pairs(vec![4, 1, 3, 3]);
        assert_eq!(actual, 5);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::count_bad_pairs(vec![1, 2, 3, 4, 5]);
        assert_eq!(actual, 0);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::count_bad_pairs(vec![1, 2, 2]);
        assert_eq!(actual, 2);
    }

    #[test]
    fn test_case4() {
        let actual = Solution::count_bad_pairs(vec![
            64, 6, 81, 7, 16, 15, 99, 47, 56, 39, 91, 85, 34, 24, 77, 99, 77, 11, 64, 63, 83, 5, 28,
        ]);
        assert_eq!(actual, 252);
    }
}
