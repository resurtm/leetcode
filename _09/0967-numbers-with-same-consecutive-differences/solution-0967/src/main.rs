use std::collections::HashSet;

impl Solution {
    #[allow(dead_code)]
    pub fn nums_same_consec_diff(max_n: i32, diff: i32) -> Vec<i32> {
        let mut res = HashSet::new();
        for curr in 1..=9 {
            Self::rec(curr, curr, 1, max_n, diff, &mut res);
        }
        res.into_iter().collect()
    }

    fn rec(curr: i32, last: i32, n: i32, max_n: i32, diff: i32, res: &mut HashSet<i32>) {
        if n == max_n {
            res.insert(curr);
            return;
        }
        if last + diff <= 9 {
            Self::rec(
                format!("{}{}", curr, last + diff).parse().unwrap(),
                last + diff,
                n + 1,
                max_n,
                diff,
                res,
            );
        }
        if last - diff >= 0 {
            Self::rec(
                format!("{}{}", curr, last - diff).parse().unwrap(),
                last - diff,
                n + 1,
                max_n,
                diff,
                res,
            );
        }
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
    use std::collections::HashSet;

    #[test]
    fn test_case1() {
        let actual = Solution::nums_same_consec_diff(3, 7);
        assert_eq!(
            HashSet::<i32>::from_iter(actual.iter().cloned()),
            HashSet::<i32>::from([181, 292, 707, 818, 929])
        );
    }

    #[test]
    fn test_case2() {
        let actual = Solution::nums_same_consec_diff(2, 1);
        assert_eq!(
            HashSet::<i32>::from_iter(actual.iter().cloned()),
            HashSet::<i32>::from([
                10, 12, 21, 23, 32, 34, 43, 45, 54, 56, 65, 67, 76, 78, 87, 89, 98
            ])
        );
    }
}
