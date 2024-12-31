impl Solution {
    pub fn find_minimum_operations(s1i: String, s2i: String, s3i: String) -> i32 {
        let mut s1 = s1i.clone();
        let mut s2 = s2i.clone();
        let mut s3 = s3i.clone();
        let mut res = 0;
        loop {
            if s1 == s2 && s1 == s3 {
                return res;
            }
            if s1.len() >= s2.len() && s1.len() >= s3.len() {
                s1.pop();
            } else if s2.len() >= s1.len() && s2.len() >= s3.len() {
                s2.pop();
            } else {
                s3.pop();
            }
            if s1.is_empty() || s2.is_empty() || s3.is_empty() {
                return -1;
            }
            res += 1;
        }
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
        let actual = Solution::find_minimum_operations(
            String::from("abc"),
            String::from("abb"),
            String::from("ab"),
        );
        assert_eq!(actual, 2);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::find_minimum_operations(
            String::from("dac"),
            String::from("bac"),
            String::from("cac"),
        );
        assert_eq!(actual, -1);
    }
}
