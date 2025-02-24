impl Solution {
    #[allow(dead_code)]
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut ord = [29usize; 28];
        for (idx, ch) in order.as_bytes().iter().enumerate() {
            ord[(ch - b'a') as usize] = idx;
        }
        let mut s = s.into_bytes();
        s.sort_by(|a, b| ord[(a - b'a') as usize].cmp(&ord[(b - b'a') as usize]));
        String::from_utf8_lossy(&s).into_owned()
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
        let actual = Solution::custom_sort_string("cba".to_string(), "abcd".to_string());
        assert_eq!(actual, "cbad");
    }

    #[test]
    fn test_case2() {
        let actual = Solution::custom_sort_string("bcafg".to_string(), "abcd".to_string());
        assert_eq!(actual, "bcad");
    }
}
