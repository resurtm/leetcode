use std::collections::{HashMap, HashSet};

impl Solution {
    #[allow(dead_code)]
    pub fn num_splits(s: String) -> i32 {
        let (mut l, mut r) = (HashMap::new(), HashMap::<u8, i32>::new());
        let (mut lu, mut ru) = (HashSet::new(), HashSet::new());
        for &ch in s.as_bytes().iter() {
            l.entry(ch)
                .and_modify(|e| {
                    *e += 1;
                })
                .or_insert(1);
            lu.insert(ch);
        }
        let mut res = 0;
        for ch in s.as_bytes() {
            l.entry(*ch).and_modify(|e| {
                *e -= 1;
            });
            if l[ch] == 0 {
                lu.remove(ch);
            }
            r.entry(*ch).and_modify(|e| {
                *e += 1;
            });
            ru.insert(ch);
            if lu.len() == ru.len() {
                res += 1;
            }
        }
        res
    }
}

struct Solution {}

fn main() {
    println!("nothing here");
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case1() {
        let actual = Solution::num_splits("aacaba".into());
        assert_eq!(actual, 2);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::num_splits("abcd".into());
        assert_eq!(actual, 1);
    }
}
