impl Solution {
    #[allow(dead_code)]
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        let (mut curr_max, mut top_max) = (0, i32::MIN);
        let (mut curr_min, mut top_min) = (0, i32::MAX);
        for num in nums {
            curr_max += num;
            top_max = top_max.max(curr_max);
            if curr_max <= 0 {
                curr_max = 0;
            }
            curr_min += num;
            top_min = top_min.min(curr_min);
            if curr_min >= 0 {
                curr_min = 0;
            }
        }
        top_max.max(top_min.abs())
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
        let actual = Solution::max_absolute_sum(vec![1, -3, 2, 3, -4]);
        assert_eq!(actual, 5);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::max_absolute_sum(vec![2, -5, 1, -4, 3, -2]);
        assert_eq!(actual, 8);
    }
}
