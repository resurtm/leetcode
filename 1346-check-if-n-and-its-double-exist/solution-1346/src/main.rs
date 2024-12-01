impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        for i in 0..arr.len() {
            for j in 0..arr.len() {
                if i == j {
                    continue;
                }
                if arr[i] == 2 * arr[j] {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case1() {
        let actual = Solution::check_if_exist(vec![10, 2, 5, 3]);
        assert_eq!(actual, true);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::check_if_exist(vec![3, 1, 7, 11]);
        assert_eq!(actual, false);
    }
}

struct Solution;

fn main() {
    println!("Hello, world!");
}
