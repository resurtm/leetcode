use std::collections::HashSet;

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let n = s.len();
        let mut l: Vec<HashSet<char>> = vec![HashSet::new(); n + 1];
        let mut r: Vec<HashSet<char>> = vec![HashSet::new(); n + 1];

        for (i, c) in s.chars().enumerate() {
            let mut t = l[i].clone();
            t.insert(c);
            l[i + 1] = t;

            let mut t = r[n - i].clone();
            t.insert(s.chars().nth(n - i - 1).unwrap());
            r[n - i - 1] = t;
        }

        let mut res: HashSet<(char, char)> = HashSet::new();
        for (i, c) in s.chars().enumerate() {
            for &t in l[i].intersection(&r[i + 1]) {
                res.insert((c, t));
            }
        }
        res.len() as i32
    }
}

struct Solution;

fn main() {
    println!(
        "1. {}",
        Solution::count_palindromic_subsequence("aabca".into()),
    );
    println!(
        "2. {}",
        Solution::count_palindromic_subsequence("adc".into()),
    );
    println!(
        "3. {}",
        Solution::count_palindromic_subsequence("bbcbaba".into()),
    );
    println!(
        "4. {}",
        Solution::count_palindromic_subsequence("aba".into()),
    );
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case1() {
        let actual = Solution::count_palindromic_subsequence("aabca".into());
        assert_eq!(actual, 3);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::count_palindromic_subsequence("adc".into());
        assert_eq!(actual, 0);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::count_palindromic_subsequence("bbcbaba".into());
        assert_eq!(actual, 4);
    }
}
