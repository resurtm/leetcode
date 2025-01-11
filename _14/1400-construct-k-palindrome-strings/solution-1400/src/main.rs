use std::collections::HashMap;

impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        if (s.len() as i32) < k {
            return false;
        }
        let fr = s.as_bytes().iter().fold(HashMap::new(), |mut a, x| {
            a.entry(x)
                .and_modify(|y| {
                    *y += 1;
                })
                .or_insert(1);
            a
        });
        k >= fr
            .iter()
            .fold(0, |a, x| if x.1 % 2 == 0 { a } else { a + 1 })
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
        let actual = Solution::can_construct("annabelle".into(), 2);
        assert!(actual);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::can_construct("leetcode".into(), 3);
        assert!(!actual);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::can_construct("true".into(), 4);
        assert!(actual);
    }
}
