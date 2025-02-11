use std::collections::VecDeque;

impl Solution {
    #[allow(dead_code)]
    pub fn remove_occurrences(str: String, part: String) -> String {
        let (str, part) = (str.as_bytes(), part.as_bytes());
        let mut q = VecDeque::new();
        for ch in str.iter() {
            q.push_back(*ch);
            loop {
                let (sl1, sl2) = q.as_slices();
                if sl1.ends_with(part) || sl2.ends_with(part) {
                    part.iter().for_each(|_| {
                        q.pop_back();
                    });
                } else {
                    break;
                }
            }
        }
        String::from_utf8_lossy(
            q.iter()
                .fold(vec![], |mut acc, x| {
                    acc.push(*x);
                    acc
                })
                .as_slice(),
        )
        .into_owned()
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
        let actual = Solution::remove_occurrences("daabcbaabcbc".into(), "abc".into());
        assert_eq!(actual, "dab");
    }

    #[test]
    fn test_case2() {
        let actual = Solution::remove_occurrences("axxxxyyyyb".into(), "xy".into());
        assert_eq!(actual, "ab");
    }
}
