impl Solution {
    pub fn max_score(s: String) -> i32 {
        let mut res = i32::MIN;
        for i in 1..s.len() {
            let mut left = s.clone();
            let right = left.split_off(i);
            let left = left.bytes().filter(|x| *x == b'0').count();
            let right = right.bytes().filter(|x| *x == b'1').count();
            res = i32::max(res, (left + right) as i32);
        }
        res
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
        let actual = Solution::max_score(String::from("011101"));
        assert_eq!(actual, 5);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::max_score(String::from("00111"));
        assert_eq!(actual, 5);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::max_score(String::from("1111"));
        assert_eq!(actual, 3);
    }

    #[test]
    fn test_case4() {
        let actual = Solution::max_score(String::from("00"));
        assert_eq!(actual, 1);
    }
}
