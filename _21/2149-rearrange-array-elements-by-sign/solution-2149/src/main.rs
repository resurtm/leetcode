impl Solution {
    #[allow(dead_code)]
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let (l, mut p, mut n, mut res) = (nums.len(), 0, 0, vec![]);
        while res.len() < l {
            while p < l && nums[p] < 0 {
                p += 1;
            }
            if p < l {
                res.push(nums[p]);
                p += 1;
            }
            while n < l && nums[n] > 0 {
                n += 1;
            }
            if n < l {
                res.push(nums[n]);
                n += 1;
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
        let actual = Solution::rearrange_array(vec![3, 1, -2, -5, 2, -4]);
        assert_eq!(actual, [3, -2, 1, -5, 2, -4]);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::rearrange_array(vec![-1, 1]);
        assert_eq!(actual, [1, -1]);
    }
}
