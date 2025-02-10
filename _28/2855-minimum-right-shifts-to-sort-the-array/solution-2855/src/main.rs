impl Solution {
    #[allow(dead_code)]
    pub fn minimum_right_shifts(nums: Vec<i32>) -> i32 {
        let len = nums.len() as i32;
        let (mut max, mut imax, mut min, mut imin) = (i32::MIN, 0, i32::MAX, 0);
        for (i, &x) in nums.iter().enumerate() {
            if max < x {
                max = x;
                imax = i as i32;
            }
            if min > x {
                min = x;
                imin = i as i32;
            }
            if i > 0 && min != x && x <= nums[i - 1] {
                return -1;
            }
        }
        if imin == 0 && imax == len - 1 {
            0
        } else if imax != imin - 1 {
            -1
        } else if nums.last().unwrap() >= nums.first().unwrap() {
            return -1;
        } else {
            len - imin
        }
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
        let actual = Solution::minimum_right_shifts(vec![3, 4, 5, 1, 2]);
        assert_eq!(actual, 2);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::minimum_right_shifts(vec![1, 3, 5]);
        assert_eq!(actual, 0);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::minimum_right_shifts(vec![2, 1, 4]);
        assert_eq!(actual, -1);
    }

    #[test]
    fn test_case4() {
        let actual = Solution::minimum_right_shifts(vec![31, 72, 79, 25]);
        assert_eq!(actual, 1);
    }

    #[test]
    fn test_case5() {
        let actual = Solution::minimum_right_shifts(vec![63, 51, 65, 87, 6, 17, 32, 14, 42, 46]);
        assert_eq!(actual, -1);
    }
}
