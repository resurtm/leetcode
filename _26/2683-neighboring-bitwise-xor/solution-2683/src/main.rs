impl Solution {
    #[allow(dead_code)]
    pub fn does_valid_array_exist(d: Vec<i32>) -> bool {
        d.iter().fold(0, |mut r, x| {
            r ^= x;
            r
        }) == 0
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
        let actual = Solution::does_valid_array_exist(vec![1, 1, 0]);
        assert!(actual);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::does_valid_array_exist(vec![1, 1]);
        assert!(actual);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::does_valid_array_exist(vec![1, 0]);
        assert!(!actual);
    }
}
