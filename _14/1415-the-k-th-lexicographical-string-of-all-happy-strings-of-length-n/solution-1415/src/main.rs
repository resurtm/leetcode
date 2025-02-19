impl Solution {
    #[allow(dead_code)]
    pub fn get_happy_string(n: i32, k: i32) -> String {
        let (mut s, mut kn) = (Vec::new(), 0);
        Self::backtrack(n, k, &mut kn, 0, &mut s).unwrap_or(String::new())
    }

    fn backtrack(n: i32, k: i32, kn: &mut i32, d: i32, s: &mut Vec<u8>) -> Option<String> {
        if n == d {
            *kn += 1;
            return if k == *kn {
                Some(String::from_utf8_lossy(s).into_owned())
            } else {
                None
            };
        }
        for ch in [b'a', b'b', b'c'].iter() {
            if s.last().unwrap_or(&0) == ch {
                continue;
            }
            s.push(*ch);
            if let Some(r) = Self::backtrack(n, k, kn, d + 1, s) {
                return Some(r);
            }
            s.pop();
        }
        None
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
        let actual = Solution::get_happy_string(1, 3);
        assert_eq!(actual, "c");
    }

    #[test]
    fn test_case2() {
        let actual = Solution::get_happy_string(1, 4);
        assert_eq!(actual, "");
    }

    #[test]
    fn test_case3() {
        let actual = Solution::get_happy_string(3, 9);
        assert_eq!(actual, "cab");
    }
}
