use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    #[allow(dead_code)]
    pub fn minimum_cost(n: Vec<i32>) -> i32 {
        let mut h = n
            .iter()
            .enumerate()
            .fold(BinaryHeap::new(), |mut a, (i, x)| {
                if i != 0 {
                    a.push(Reverse(x));
                }
                a
            });
        n[0] + h.pop().unwrap().0 + h.pop().unwrap().0
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
        let actual = Solution::minimum_cost(vec![1, 2, 3, 12]);
        assert_eq!(actual, 6);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::minimum_cost(vec![5, 4, 3]);
        assert_eq!(actual, 12);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::minimum_cost(vec![10, 3, 1, 1]);
        assert_eq!(actual, 12);
    }
}
