use std::collections::HashSet;

impl Solution {
    pub fn maximum_length(sraw: String) -> i32 {
        let mut res = -1i32;
        let s: Vec<char> = sraw.chars().into_iter().collect();
        for size in 1..s.len() {
            for it in s.windows(size) {
                let mut set = HashSet::new();
                it.iter().for_each(|x| {
                    set.insert(x);
                });
                if set.len() > 1 {
                    continue;
                }
                let count = sraw
                    .chars()
                    .collect::<Vec<char>>()
                    .windows(size)
                    .filter(|&x| x == it)
                    .count();
                if count >= 3 && res < size as i32 {
                    res = size as i32;
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_case1() {
        let actual = Solution::maximum_length(String::from("aaaa"));
        assert_eq!(actual, 2);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::maximum_length(String::from("abcdef"));
        assert_eq!(actual, -1);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::maximum_length(String::from("abcaba"));
        assert_eq!(actual, 1);
    }

    #[test]
    fn test_case4() {
        let actual = Solution::maximum_length(String::from("fafff"));
        assert_eq!(actual, 1);
    }

    #[test]
    fn test_case5() {
        let actual = Solution::maximum_length(String::from(
            "cccerrrecdcdccedecdcccddeeeddcdcddedccdceeedccecde",
        ));
        assert_eq!(actual, 2);
    }

    #[test]
    fn test_case6() {
        let actual = Solution::maximum_length(String::from("abcccccdddd"));
        assert_eq!(actual, 3);
    }
}

struct Solution;

fn main() {
    println!("Hello, world!");
}
