use std::collections::HashSet;

impl Solution {
    #[allow(dead_code)]
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let sz = nums[0].len();
        let nums: HashSet<_> = nums.into_iter().collect();
        Self::backtrack(sz, &nums, String::new()).unwrap_or(String::new())
    }

    fn backtrack(sz: usize, nums: &HashSet<String>, acc: String) -> Option<String> {
        if sz == acc.len() {
            return if nums.contains(&acc) { None } else { Some(acc) };
        }
        if let Some(x) = Self::backtrack(sz, nums, format!("{}{}", acc, "0")) {
            return Some(x);
        }
        if let Some(x) = Self::backtrack(sz, nums, format!("{}{}", acc, "1")) {
            return Some(x);
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
        let actual = Solution::find_different_binary_string(vec!["01".into(), "10".into()]);
        assert!(["11".into(), "00".into()].contains(&actual));
    }

    #[test]
    fn test_case2() {
        let actual = Solution::find_different_binary_string(vec!["00".into(), "01".into()]);
        assert!(["11".into(), "10".into()].contains(&actual));
    }

    #[test]
    fn test_case3() {
        let actual =
            Solution::find_different_binary_string(vec!["111".into(), "011".into(), "001".into()]);
        assert!([
            "101".into(),
            "000".into(),
            "010".into(),
            "100".into(),
            "110".into()
        ]
        .contains(&actual));
    }
}
