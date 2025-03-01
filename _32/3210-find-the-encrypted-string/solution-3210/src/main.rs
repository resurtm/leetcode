impl Solution {
    #[allow(dead_code)]
    pub fn get_encrypted_string(s: String, k: i32) -> String {
        let (s, mut r) = (s.chars().collect::<Vec<char>>(), String::new());
        for i in 0..s.len() {
            r.push(s[(i + k as usize) % s.len()]);
        }
        r
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
        let actual = Solution::get_encrypted_string("dart".into(), 3);
        assert_eq!(actual, "tdar");
    }

    #[test]
    fn test_case2() {
        let actual = Solution::get_encrypted_string("aaa".into(), 1);
        assert_eq!(actual, "aaa");
    }
}
