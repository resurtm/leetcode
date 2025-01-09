impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        words
            .iter()
            .fold(0, |a, x| if x.starts_with(&pref) { a + 1 } else { a })
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
        let actual = Solution::prefix_count(
            vec![
                "pay".into(),
                "attention".into(),
                "practice".into(),
                "attend".into(),
            ],
            "at".into(),
        );
        assert_eq!(actual, 2);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::prefix_count(
            vec![
                "leetcode".into(),
                "win".into(),
                "loops".into(),
                "success".into(),
            ],
            "code".into(),
        );
        assert_eq!(actual, 0);
    }
}
