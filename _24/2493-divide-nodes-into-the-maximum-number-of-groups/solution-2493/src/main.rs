use std::collections::{HashMap, VecDeque};

impl Solution {
    /// Based on this:
    /// https://leetcode.com/problems/divide-nodes-into-the-maximum-number-of-groups/editorial/
    #[allow(dead_code)]
    pub fn magnificent_sets(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut adj_list = vec![vec![]; n as usize];
        let mut parent = vec![-1; n as usize];
        let mut depth = vec![0; n as usize];
        edges.iter().for_each(|x| {
            let (node1, node2) = (x[0], x[1]);
            adj_list[node1 as usize - 1].push(node2 - 1);
            adj_list[node2 as usize - 1].push(node1 - 1);
            Self::union(node1 - 1, node2 - 1, &mut parent, &mut depth);
        });
        let mut num_groups_node: HashMap<i32, i32> = HashMap::new();
        for node in 0..n {
            let num_groups = Self::num_groups(&adj_list, node, n);
            if num_groups == -1 {
                return -1;
            }
            let root = Self::find(node, &mut parent);
            num_groups_node.insert(
                root,
                num_groups.max(*num_groups_node.get(&root).unwrap_or(&0)),
            );
        }
        num_groups_node.values().sum()
    }

    fn num_groups(adj_list: &[Vec<i32>], node: i32, n: i32) -> i32 {
        let mut queue = VecDeque::from([node]);
        let mut viz = vec![-1; n as usize];
        viz[node as usize] = 0;
        let mut deepest = 0;
        while !queue.is_empty() {
            for &curr in std::mem::take(&mut queue).iter() {
                for &neighbor in adj_list[curr as usize].iter() {
                    if viz[neighbor as usize] == -1 {
                        viz[neighbor as usize] = deepest + 1;
                        queue.push_back(neighbor);
                    } else if viz[neighbor as usize] == deepest {
                        return -1;
                    }
                }
            }
            deepest += 1;
        }
        deepest
    }

    fn union(node1: i32, node2: i32, parent: &mut [i32], depth: &mut [i32]) {
        let (mut node1, mut node2) = (Self::find(node1, parent), Self::find(node2, parent));
        if node1 == node2 {
            return;
        }
        if depth[node1 as usize] < depth[node2 as usize] {
            (node1, node2) = (node2, node1);
        }
        parent[node2 as usize] = node1;
        if depth[node1 as usize] == depth[node2 as usize] {
            depth[node1 as usize] += 1;
        }
    }

    fn find(node: i32, parent: &mut [i32]) -> i32 {
        let mut res = node;
        while parent[res as usize] != -1 {
            res = parent[res as usize];
        }
        res
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
        let actual = Solution::magnificent_sets(
            6,
            vec![
                vec![1, 2],
                vec![1, 4],
                vec![1, 5],
                vec![2, 6],
                vec![2, 3],
                vec![4, 6],
            ],
        );
        assert_eq!(actual, 4);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::magnificent_sets(3, vec![vec![1, 2], vec![2, 3], vec![3, 1]]);
        assert_eq!(actual, -1);
    }
}
