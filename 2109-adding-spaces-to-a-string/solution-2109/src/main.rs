impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        let mut res = String::new();
        let mut pos = 0 as usize;
        for (idx, chr) in s.chars().enumerate() {
            if pos < spaces.len() && idx as i32 == spaces[pos] {
                pos += 1;
                res.push(' ');
            }
            res.push(chr);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case1() {
        let actual = Solution::add_spaces(String::from("LeetcodeHelpsMeLearn"), vec![8, 13, 15]);
        assert_eq!(actual, "Leetcode Helps Me Learn");
    }

    #[test]
    fn test_case2() {
        let actual = Solution::add_spaces(String::from("icodeinpython"), vec![1, 5, 7, 9]);
        assert_eq!(actual, "i code in py thon");
    }

    #[test]
    fn test_case3() {
        let actual = Solution::add_spaces(String::from("spacing"), vec![0, 1, 2, 3, 4, 5, 6]);
        assert_eq!(actual, " s p a c i n g");
    }
}

struct Solution;

fn main() {
    println!("Hello, world!");
}
