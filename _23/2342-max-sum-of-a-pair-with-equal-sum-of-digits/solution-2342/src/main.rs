use std::collections::{BinaryHeap, HashMap};

impl Solution {
    #[allow(dead_code)]
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let mut cache: HashMap<i32, BinaryHeap<i32>> = HashMap::new();
        let mut res = -1;
        for &num in nums.iter() {
            let sum = num
                .to_string()
                .as_bytes()
                .iter()
                .fold(0, |acc, x| acc + (*x - b'0') as i32);
            cache
                .entry(sum)
                .and_modify(|e| {
                    e.push(num);
                    if e.len() == 3 {
                        let (el1, el2, _) = (e.pop().unwrap(), e.pop().unwrap(), e.pop().unwrap());
                        e.push(el1);
                        e.push(el2);
                    }
                    if e.len() == 2 {
                        let (el1, el2) = (e.pop().unwrap(), e.pop().unwrap());
                        res = res.max(el1 + el2);
                        e.push(el1);
                        e.push(el2);
                    }
                })
                .or_insert(BinaryHeap::from([num]));
        }
        res
    }
}

#[allow(dead_code)]
struct Solution {}

fn main() {
    println!("nothing here");
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case1() {
        let actual = Solution::maximum_sum(vec![18, 43, 36, 13, 7]);
        assert_eq!(actual, 54);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::maximum_sum(vec![10, 12, 19, 14]);
        assert_eq!(actual, -1);
    }
}
