impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        sentence
            .split(" ")
            .into_iter()
            .enumerate()
            .fold(-1, |acc, (idx, word)| {
                if acc != -1 {
                    acc
                } else if word.starts_with(&search_word) {
                    idx as i32 + 1
                } else {
                    -1
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_case1() {
        let actual =
            Solution::is_prefix_of_word(String::from("i love eating burger"), String::from("burg"));
        assert_eq!(actual, 4);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::is_prefix_of_word(
            String::from("this problem is an easy problem"),
            String::from("pro"),
        );
        assert_eq!(actual, 2);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::is_prefix_of_word(String::from("i am tired"), String::from("you"));
        assert_eq!(actual, -1);
    }
}

struct Solution;

fn main() {
    println!("Hello, world!");
}
