use std::collections::HashMap;

impl Solution {
    #[allow(dead_code)]
    pub fn minimum_length(si: String) -> i32 {
        let mut fr = HashMap::new();
        si.as_bytes().iter().for_each(|x| {
            fr.entry(x).and_modify(|y| *y += 1).or_insert(1);
        });
        let mut res = 0;
        for i in b'a'..=b'z' {
            if let Some(&c) = fr.get(&i) {
                res += if c % 2 == 0 { 2 } else { 1 };
            }
        }
        res
    }
}

#[allow(dead_code)]
struct Solution;

fn main() {
    println!("nothing here");
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case1() {
        let actual = Solution::minimum_length("abaacbcbb".into());
        assert_eq!(actual, 5);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::minimum_length("aa".into());
        assert_eq!(actual, 2);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::minimum_length("lyqkwhyy".into());
        assert_eq!(actual, 6);
    }
}
