use std::collections::{HashMap, HashSet};

impl Solution {
    #[allow(dead_code)]
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut links = HashMap::new();
        let mut res = vec![];
        for edge in edges.iter() {
            let (a, b) = (edge[0], edge[1]);
            if Self::bfs(a, b, &links) {
                res = vec![a, b];
            }
            links
                .entry(a)
                .and_modify(|v: &mut Vec<i32>| {
                    v.push(b);
                })
                .or_insert(vec![b]);
            links
                .entry(b)
                .and_modify(|v: &mut Vec<i32>| {
                    v.push(a);
                })
                .or_insert(vec![a]);
        }
        res
    }

    fn bfs(a: i32, b: i32, links: &HashMap<i32, Vec<i32>>) -> bool {
        let mut q = vec![a];
        let mut v = HashSet::from([a]);
        while !q.is_empty() {
            if let Some(c) = q.pop() {
                if c == b {
                    return true;
                }
                for link in links.get(&c).unwrap_or(&vec![]).iter() {
                    if !v.contains(link) {
                        q.push(*link);
                        v.insert(*link);
                    }
                }
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
        let actual = Solution::find_redundant_connection(vec![vec![1, 2], vec![1, 3], vec![2, 3]]);
        assert_eq!(actual, [2, 3]);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::find_redundant_connection(vec![
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![1, 4],
            vec![1, 5],
        ]);
        assert_eq!(actual, [1, 4]);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::find_redundant_connection(vec![
            vec![9, 10],
            vec![5, 8],
            vec![2, 6],
            vec![1, 5],
            vec![3, 8],
            vec![4, 9],
            vec![8, 10],
            vec![4, 10],
            vec![6, 8],
            vec![7, 9],
        ]);
        assert_eq!(actual, [4, 10]);
    }
}
