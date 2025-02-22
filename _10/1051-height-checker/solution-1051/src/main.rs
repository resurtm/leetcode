impl Solution {
    #[allow(dead_code)]
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut h = heights.clone();
        h.sort();
        let mut res = 0;
        for i in 0..h.len() {
            if h[i] != heights[i] {
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
        let actual = Solution::height_checker(vec![1, 1, 4, 2, 1, 3]);
        assert_eq!(actual, 3);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::height_checker(vec![5, 1, 2, 3, 4]);
        assert_eq!(actual, 5);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::height_checker(vec![1, 2, 3, 4, 5]);
        assert_eq!(actual, 0);
    }
}
