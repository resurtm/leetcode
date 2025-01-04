use std::collections::HashSet;

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let mut l = [usize::MAX; 26];
        let mut r = [usize::MIN; 26];

        for (i, cr) in s.bytes().enumerate() {
            let c = (cr - b'a') as usize;
            l[c] = usize::min(l[c], i);
            r[c] = usize::max(r[c], i);
        }

        let mut res = 0;
        for cr in b'a'..=b'z' {
            let c = (cr - b'a') as usize;
            if l[c] < r[c] {
                let t: HashSet<u8> = HashSet::from_iter(s[l[c] + 1..r[c]].bytes());
                res += t.len() as i32;
            }
        }
        res
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
        Solution::count_palindromic_subsequence(
            "tlpjzdmtwderpkpmgoyrcxttiheassztncqvnfjeyxxp".into()
        ),
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

    #[test]
    fn test_case4() {
        let actual = Solution::count_palindromic_subsequence(
            "tlpjzdmtwderpkpmgoyrcxttiheassztncqvnfjeyxxp".into(),
        );
        assert_eq!(actual, 161);
    }
}
