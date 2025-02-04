impl Solution {
    #[allow(dead_code)]
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        if nums.len() == 1 {
            return nums[0];
        }
        let (mut sum, mut res) = (nums[0], nums[0]);
        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                sum += nums[i];
            } else {
                sum = nums[i];
            }
            res = res.max(sum)
        }
        res
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
        let actual = Solution::max_ascending_sum(vec![10, 20, 30, 5, 10, 50]);
        assert_eq!(actual, 65);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::max_ascending_sum(vec![10, 20, 30, 40, 50]);
        assert_eq!(actual, 150);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::max_ascending_sum(vec![12, 17, 15, 13, 10, 11, 12]);
        assert_eq!(actual, 33);
    }
}
