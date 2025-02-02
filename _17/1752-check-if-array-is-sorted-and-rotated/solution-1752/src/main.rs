impl Solution {
    #[allow(dead_code)]
    pub fn check(nums: Vec<i32>) -> bool {
        let len = nums.len();
        for i in 0..len {
            let mut sorted = true;
            for j in 0..len - 1 {
                if nums[(i + j) % len] > nums[(i + j + 1) % len] {
                    sorted = false;
                    break;
                }
            }
            if sorted {
                return true;
            }
        }
        false
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
        let actual = Solution::check(vec![3, 4, 5, 1, 2]);
        assert!(actual);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::check(vec![2, 1, 3, 4]);
        assert!(!actual);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::check(vec![1, 2, 3]);
        assert!(actual);
    }

    #[test]
    fn test_case4() {
        let actual = Solution::check(vec![3, 1, 2, 2, 4]);
        assert!(!actual);
    }

    #[test]
    fn test_case5() {
        let actual = Solution::check(vec![2, 4, 1, 3]);
        assert!(!actual);
    }
}
