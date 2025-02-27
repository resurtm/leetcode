use std::collections::HashMap;

impl Solution {
    #[allow(dead_code)]
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        let (n, mut idx) = (arr.len(), HashMap::<i32, usize>::new());
        arr.iter().enumerate().for_each(|(i, &x)| {
            idx.insert(x, i);
        });
        let (mut res, mut dp) = (0, vec![vec![0; n]; n]);
        for i in 0..n {
            for j in 0..i {
                let diff = arr[i] - arr[j];
                if let Some(&d) = idx.get(&diff) {
                    if d < j {
                        dp[j][i] = dp[d][j] + 1;
                    } else {
                        dp[j][i] = 2;
                    }
                } else {
                    dp[j][i] = 2;
                }
                res = res.max(dp[j][i]);
            }
        }
        if res > 2 { res } else { 0 }
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
        let actual = Solution::len_longest_fib_subseq(vec![1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(actual, 5);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::len_longest_fib_subseq(vec![1, 3, 7, 11, 12, 14, 18]);
        assert_eq!(actual, 3);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::len_longest_fib_subseq(vec![1, 2, 3]);
        assert_eq!(actual, 3);
    }

    #[test]
    fn test_case4() {
        let actual = Solution::len_longest_fib_subseq(vec![1, 3, 5]);
        assert_eq!(actual, 0);
    }
}
