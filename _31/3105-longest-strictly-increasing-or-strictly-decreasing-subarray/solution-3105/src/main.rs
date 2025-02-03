impl Solution {
    #[allow(dead_code)]
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let (mut st, mut len, mut res) = (0i8, 1, 1);
        for i in 0..nums.len() - 1 {
            if st == 0 {
                if nums[i + 1] > nums[i] {
                    st = 1;
                    len = 2;
                } else if nums[i + 1] < nums[i] {
                    st = -1;
                    len = 2;
                } else if nums[i + 1] == nums[i] {
                    len = 1;
                }
            } else if st == 1 {
                if nums[i + 1] == nums[i] {
                    res = res.max(len);
                    st = 0;
                    len = 1;
                } else if nums[i + 1] < nums[i] {
                    res = res.max(len);
                    st = -1;
                    len = 2;
                } else if nums[i + 1] > nums[i] {
                    len += 1;
                }
            } else if st == -1 {
                if nums[i + 1] == nums[i] {
                    res = res.max(len);
                    st = 0;
                    len = 1;
                } else if nums[i + 1] > nums[i] {
                    res = res.max(len);
                    st = 1;
                    len = 2;
                } else if nums[i + 1] < nums[i] {
                    len += 1;
                }
            }
        }
        res.max(len)
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
        let actual = Solution::longest_monotonic_subarray(vec![1, 4, 3, 3, 2]);
        assert_eq!(actual, 2);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::longest_monotonic_subarray(vec![3, 3, 3, 3]);
        assert_eq!(actual, 1);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::longest_monotonic_subarray(vec![3, 2, 1]);
        assert_eq!(actual, 3);
    }
}
