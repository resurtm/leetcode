impl Solution {
    #[allow(dead_code)]
    pub fn count_matching_subarrays(nums: Vec<i32>, pattern: Vec<i32>) -> i32 {
        let (mut res, mut acc) = (0, vec![]);
        for idx in 0..nums.len() {
            if idx < nums.len() - pattern.len() {
                acc.push((idx, 0));
            }
            for (n, p) in std::mem::take(&mut acc) {
                if p == pattern.len() {
                    res += 1;
                    continue;
                }
                if pattern[p] == 1 && nums[idx + 1] > nums[idx]
                    || pattern[p] == 0 && nums[idx + 1] == nums[idx]
                    || pattern[p] == -1 && nums[idx + 1] < nums[idx]
                {
                    acc.push((n + 1, p + 1));
                }
            }
        }
        res
    }
}

struct Solution {}

fn main() {
    println!("nothing here");
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case1() {
        let actual = Solution::count_matching_subarrays(vec![1, 2, 3, 4, 5, 6], vec![1, 1]);
        assert_eq!(actual, 4);
    }

    #[test]
    fn test_case2() {
        let actual =
            Solution::count_matching_subarrays(vec![1, 4, 4, 1, 3, 5, 5, 3], vec![1, 0, -1]);
        assert_eq!(actual, 2);
    }
}
