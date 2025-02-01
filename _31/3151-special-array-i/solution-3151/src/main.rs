impl Solution {
    #[allow(dead_code)]
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        if nums.len() == 1 {
            return true;
        }
        for i in 0..nums.len() - 1 {
            let (a, b) = (nums[i] % 2 == 0, nums[i + 1] % 2 == 0);
            if a && b || !a && !b {
                return false;
            }
        }
        true
    }
}

#[allow(dead_code)]
struct Solution;

fn main() {
    println!("nothing here");
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case1() {
        let actual = Solution::is_array_special(vec![1]);
        assert_eq!(actual, true);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::is_array_special(vec![2, 1, 4]);
        assert_eq!(actual, true);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::is_array_special(vec![4, 3, 1, 6]);
        assert_eq!(actual, false);
    }
}
