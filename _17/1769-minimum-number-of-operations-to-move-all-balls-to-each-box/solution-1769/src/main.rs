impl Solution {
    pub fn min_operations(its: String) -> Vec<i32> {
        let n = its.len();
        let mut l = vec![0; n];
        let mut r = vec![0; n];
        let bs = its.as_bytes();
        for i in 0..n {
            let mut s1 = 0;
            let mut s2 = 0;
            (0..i).for_each(|j| {
                s1 += (bs[j] - b'0') as i32 * (i - j) as i32;
            });
            (i..n).for_each(|j| {
                s2 += (bs[j] - b'0') as i32 * (j - i) as i32;
            });
            l[i] = s1;
            r[i] = s2;
        }
        (0..n).map(|i| l[i] + r[i]).collect()
    }
}

struct Solution;

fn main() {
    println!("nothing here");
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case1() {
        let actual = Solution::min_operations("110".into());
        assert_eq!(actual, vec![1, 1, 3]);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::min_operations("001011".into());
        assert_eq!(actual, vec![11, 8, 5, 4, 3, 4]);
    }
}
