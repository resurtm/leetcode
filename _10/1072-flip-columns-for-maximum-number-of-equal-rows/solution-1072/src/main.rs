impl Solution {
    pub fn max_equal_rows_after_flips(mat: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        for bin_row in &mat {
            let mut inv_bin = Vec::new();
            for val in bin_row {
                inv_bin.push(1 - val);
            }
            let mut tmp = 0;
            for cur_row in &mat {
                if cur_row == bin_row || cur_row == &inv_bin {
                    tmp += 1;
                }
            }
            ans = std::cmp::max(ans, tmp);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case1() {
        let actual =
            Solution::max_equal_rows_after_flips(vec![vec![0, 0, 0], vec![0, 0, 1], vec![1, 1, 0]]);
        assert_eq!(actual, 2);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::max_equal_rows_after_flips(vec![vec![0, 1], vec![1, 0]]);
        assert_eq!(actual, 2);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::max_equal_rows_after_flips(vec![vec![0, 1], vec![1, 1]]);
        assert_eq!(actual, 1);
    }
}

struct Solution;

fn main() {
    println!("Hello, world!");
}
