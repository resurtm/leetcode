use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(PartialEq, PartialOrd, Debug)]
struct Wrapper(f64);

impl Ord for Wrapper {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse the comparison for a min-heap
        self.0.partial_cmp(&other.0).unwrap()
    }
}

impl Eq for Wrapper {}

fn calc(p: i32, n: i32) -> (Wrapper, i32, i32) {
    let x = n as f64;
    (Wrapper((n - p) as f64 / (x + x * x)), p, n)
}

impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        let mut heap: BinaryHeap<(Wrapper, i32, i32)> = classes
            .into_iter()
            .map((|v| calc(v[0], v[1])))
            .collect::<BinaryHeap<_>>();
        for _ in 0..extra_students {
            let Some((_, n, p)) = heap.pop() else {
                panic!()
            };
            heap.push(calc(n + 1, p + 1))
        }
        let k = heap.len();
        heap.into_iter()
            .map(|(_, n, k)| n as f64 / k as f64)
            .sum::<f64>()
            / k as f64
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case1() {
        assert_eq!(
            Solution::max_average_ratio(vec![vec![1, 2], vec![3, 5], vec![2, 2]], 2),
            0.78333
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            Solution::max_average_ratio(vec![vec![2, 4], vec![3, 9], vec![4, 5], vec![2, 10]], 4),
            0.53485
        );
    }
}

struct Solution;

fn main() {
    println!("Hello, world!");
}
