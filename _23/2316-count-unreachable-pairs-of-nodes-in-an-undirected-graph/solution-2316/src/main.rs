use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    #[allow(dead_code)]
    pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        let mut links: HashMap<i32, Vec<i32>> = HashMap::new();
        for edge in edges.iter() {
            links
                .entry(edge[0])
                .and_modify(|e| {
                    e.push(edge[1]);
                })
                .or_insert(vec![edge[1]]);
            links
                .entry(edge[1])
                .and_modify(|e| {
                    e.push(edge[0]);
                })
                .or_insert(vec![edge[0]]);
        }
        let mut viz = HashSet::new();
        let (mut res, mut sum) = (0, n as i64);
        for nx in 0..n {
            if viz.contains(&nx) {
                continue;
            }
            viz.insert(nx);
            let (mut q, mut s) = (VecDeque::new(), HashSet::new());
            q.push_back(nx);
            s.insert(nx);
            while let Some(c) = q.pop_front() {
                for &link in links.get(&c).unwrap_or(&vec![]).iter() {
                    if !viz.contains(&link) {
                        viz.insert(link);
                        q.push_back(link);
                        s.insert(link);
                    }
                }
            }
            sum -= s.len() as i64;
            res += s.len() as i64 * sum;
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
        let actual = Solution::count_pairs(
            7,
            vec![vec![0, 2], vec![0, 5], vec![2, 4], vec![1, 6], vec![5, 4]],
        );
        assert_eq!(actual, 14);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::count_pairs(
            11,
            vec![
                vec![5, 0],
                vec![1, 0],
                vec![10, 7],
                vec![9, 8],
                vec![7, 2],
                vec![1, 3],
                vec![0, 2],
                vec![8, 5],
                vec![4, 6],
                vec![4, 2],
            ],
        );
        assert_eq!(actual, 0);
    }
}
