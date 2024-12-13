use std::collections::HashMap;

impl Solution {
    pub fn rearrange_characters(s: String, t: String) -> i32 {
        let s_fr = Self::build_freq(s);
        let t_fr = Self::build_freq(t);
        t_fr.iter().fold(-1, |acc, (ch, num)| {
            let times = s_fr.get(ch).unwrap_or(&0) / num;
            if acc == -1 || acc > times {
                times
            } else {
                acc
            }
        })
    }

    fn build_freq(s: String) -> HashMap<char, i32> {
        s.chars().into_iter().fold(HashMap::new(), |mut acc, x| {
            *acc.entry(x).or_insert(0) += 1;
            acc
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case1() {
        assert_eq!(
            Solution::rearrange_characters(
                String::from("ilovecodingonleetcode"),
                String::from("code")
            ),
            2
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            Solution::rearrange_characters(String::from("abcba"), String::from("abc")),
            1
        );
    }

    #[test]
    fn test_case3() {
        assert_eq!(
            Solution::rearrange_characters(String::from("abbaccaddaeea"), String::from("aaaaa")),
            1
        );
    }
}

struct Solution;

fn main() {
    println!("Hello, world!");
}
