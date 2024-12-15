impl Solution {
    pub fn continuous_subarrays(nums: Vec<i32>) -> i64 {
        let mut res = 0;
        if nums.is_empty() {
            return res;
        }
        for sz in 1..nums.len() + 1 {
            for win in nums.windows(sz) {
                let mut good = true;
                'i: for i in 0..win.len() {
                    for j in i..win.len() {
                        let v = (win[i] - win[j]).abs();
                        if v > 2 {
                            good = false;
                            break 'i;
                        }
                    }
                }
                if good {
                    res += 1;
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case1() {
        assert_eq!(Solution::continuous_subarrays(vec![5, 4, 2, 4],), 8);
    }

    #[test]
    fn test_case2() {
        assert_eq!(Solution::continuous_subarrays(vec![1, 2, 3],), 6);
    }
}

struct Solution;

fn main() {
    println!("Hello, world!");
}
