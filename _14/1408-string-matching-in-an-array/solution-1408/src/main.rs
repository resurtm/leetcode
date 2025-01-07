impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        words.iter().fold(vec![], |mut acc, x| {
            if words.iter().find(|&y| y.contains(x) && x != y).is_some() {
                acc.push(x.clone());
            }
            acc
        })
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
        let actual = Solution::string_matching(vec![
            "mass".into(),
            "as".into(),
            "hero".into(),
            "superhero".into(),
        ]);
        assert_eq!(actual, vec![String::from("as"), String::from("hero")]);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::string_matching(vec!["leetcode".into(), "et".into(), "code".into()]);
        assert_eq!(actual, vec![String::from("et"), String::from("code")]);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::string_matching(vec!["blue".into(), "green".into(), "bu".into()]);
        assert_eq!(actual, Vec::<String>::new());
    }
}
