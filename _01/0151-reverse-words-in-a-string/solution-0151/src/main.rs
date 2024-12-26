impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut parts: Vec<&str> = s
            .split(" ")
            .into_iter()
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .collect();
        parts.reverse();
        parts.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case1() {
        let actual = Solution::reverse_words(String::from("the sky is blue"));
        assert_eq!(actual, String::from("blue is sky the"));
    }

    #[test]
    fn test_case2() {
        let actual = Solution::reverse_words(String::from("  hello world  "));
        assert_eq!(actual, String::from("world hello"));
    }

    #[test]
    fn test_case3() {
        let actual = Solution::reverse_words(String::from("a good   example"));
        assert_eq!(actual, String::from("example good a"));
    }
}

struct Solution;

fn main() {
    println!("Hello, world!");
}
