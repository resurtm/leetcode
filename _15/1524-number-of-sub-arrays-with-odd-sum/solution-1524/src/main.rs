impl Solution {
    #[allow(dead_code)]
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        let (n, m) = (arr.len(), 1_000_000_000 + 7);
        let arr: Vec<_> = arr.iter().map(|x| *x % 2).collect();
        let (mut dp_zero, mut dp_one) = (vec![0; n], vec![0; n]);
        if arr[n - 1] == 0 {
            dp_zero[n - 1] = 1;
        } else {
            dp_one[n - 1] = 1;
        }
        for (i, x) in arr.iter().enumerate().rev() {
            if i == n - 1 {
                continue;
            }
            if *x == 1 {
                dp_one[i] = (1 + dp_zero[i + 1]) % m;
                dp_zero[i] = dp_one[i + 1];
            } else {
                dp_zero[i] = (1 + dp_zero[i + 1]) % m;
                dp_one[i] = dp_one[i + 1];
            }
        }
        dp_one.iter().fold(0, |acc, x| (acc + x) % m)
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
        let actual = Solution::num_of_subarrays(vec![1, 3, 5]);
        assert_eq!(actual, 4);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::num_of_subarrays(vec![2, 4, 6]);
        assert_eq!(actual, 0);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::num_of_subarrays(vec![1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(actual, 16);
    }
}
