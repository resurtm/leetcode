impl Solution {
    #[allow(dead_code)]
    pub fn apply_operations(nums: Vec<i32>) -> Vec<i32> {
        let (mut nums, len) = (nums.clone(), nums.len());
        let (mut res1, mut res2) = (Vec::with_capacity(len), Vec::with_capacity(len));
        for i in 0..nums.len() {
            if i != len - 1 && nums[i] == nums[i + 1] {
                nums[i] *= 2;
                nums[i + 1] = 0;
            }
            if nums[i] == 0 {
                res2.push(nums[i]);
            } else {
                res1.push(nums[i]);
            }
        }
        res1.extend(res2);
        res1
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
        let actual = Solution::apply_operations(vec![1, 2, 2, 1, 1, 0]);
        assert_eq!(actual, [1, 4, 2, 0, 0, 0]);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::apply_operations(vec![0, 1]);
        assert_eq!(actual, [1, 0]);
    }
}
