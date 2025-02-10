impl Solution {
    #[allow(dead_code)]
    pub fn clear_digits(s: String) -> String {
        let mut s = s.clone();
        let mut i = 0;
        while !s.is_empty() && i < s.len() as i32 {
            if let Some(ch) = s.chars().nth(i as usize) {
                if ch.is_ascii_digit() {
                    s.remove(i as usize);
                    s.remove(i as usize - 1);
                    i -= 2;
                }
            }
            i += 1;
        }
        s
    }
}

#[allow(dead_code)]
struct Solution {}

fn main() {
    println!("nothing here");
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case1() {
        let actual = Solution::clear_digits("abc".into());
        assert_eq!(actual, "abc");
    }

    #[test]
    fn test_case2() {
        let actual = Solution::clear_digits("cb34".into());
        assert_eq!(actual, "");
    }
}
