use std::collections::HashMap;

impl Solution {
    #[allow(dead_code)]
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        let mut f = HashMap::new();
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                let (a, b) = (nums[i], nums[j]);
                f.entry(a * b).and_modify(|e| *e += 2).or_insert(2);
            }
        }
        f.iter().fold(0, |acc, x| {
            if *x.1 >= 4 {
                acc + *x.1 * (*x.1 - 2)
            } else {
                acc
            }
        })
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
        let actual = Solution::tuple_same_product(vec![2, 3, 4, 6]);
        assert_eq!(actual, 8);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::tuple_same_product(vec![1, 2, 4, 5, 10]);
        assert_eq!(actual, 16);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::tuple_same_product(vec![2, 3, 4, 6, 8, 12]);
        assert_eq!(actual, 40);
    }
}
