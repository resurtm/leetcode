use std::collections::HashMap;

impl Solution {
    pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
        let mut a = HashMap::new();
        target.iter().for_each(|x| {
            a.entry(*x)
                .and_modify(|y| {
                    *y += 1;
                })
                .or_insert(1);
        });
        let mut b = HashMap::new();
        arr.iter().for_each(|x| {
            b.entry(*x)
                .and_modify(|y| {
                    *y += 1;
                })
                .or_insert(1);
        });
        a == b
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
        let actual = Solution::can_be_equal(vec![1, 2, 3, 4], vec![2, 4, 1, 3]);
        assert!(actual);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::can_be_equal(vec![7], vec![7]);
        assert!(actual);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::can_be_equal(vec![3, 7, 9], vec![3, 7, 11]);
        assert!(!actual);
    }
}
