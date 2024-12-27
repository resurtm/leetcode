use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut nodes = HashSet::new();
        pairs.iter().for_each(|x| {
            nodes.insert(x[0]);
            nodes.insert(x[1]);
        });

        let mut out: HashMap<i32, i32> = HashMap::new();
        let mut inc: HashMap<i32, i32> = HashMap::new();
        let mut adj: HashMap<i32, Vec<i32>> = HashMap::new();
        nodes.iter().for_each(|&x| {
            out.insert(x, 0);
            inc.insert(x, 0);
            adj.insert(x, Vec::new());
        });
        pairs.iter().for_each(|x| {
            out.insert(x[0], *out.get(&x[0]).unwrap() + 1);
            inc.insert(x[1], *inc.get(&x[1]).unwrap() + 1);
            adj.entry(x[0]).or_insert_with(Vec::new).push(x[1]);
        });

        let mut start = -1;
        nodes.iter().for_each(|x| {
            if out[x] - inc[x] >= 1 {
                start = *x;
                return;
            }
        });
        if start == -1 {
            start = pairs[0][0];
        }

        let mut dfs = Dfs {
            out,
            adj,
            path: Vec::new(),
        };
        dfs.run(start);

        let mut res = Vec::new();
        for node in dfs.path.iter().rev() {
            res.push(*node);
            res.push(*node);
        }
        res.remove(0);
        res.remove(res.len() - 1);

        let mut fin = Vec::new();
        for chunk in res.chunks(2) {
            fin.push(chunk.to_vec());
        }
        fin
    }
}

struct Dfs {
    out: HashMap<i32, i32>,
    adj: HashMap<i32, Vec<i32>>,
    path: Vec<i32>,
}

impl Dfs {
    fn run(&mut self, curr: i32) {
        while *self.out.get(&curr).unwrap() != 0 {
            let idx = *self.out.get(&curr).unwrap() - 1;
            self.out.insert(curr, idx);
            let next = self.adj.get(&curr).unwrap().get(idx as usize).unwrap();
            self.run(*next);
        }
        self.path.push(curr);
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case1() {
        let actual =
            Solution::valid_arrangement(vec![vec![5, 1], vec![4, 5], vec![11, 9], vec![9, 4]]);
        assert_eq!(
            actual,
            vec![vec![11, 9], vec![9, 4], vec![4, 5], vec![5, 1]]
        );
    }

    #[test]
    fn test_case2() {
        let actual = Solution::valid_arrangement(vec![vec![1, 3], vec![3, 2], vec![2, 1]]);
        assert_eq!(actual, vec![vec![1, 3], vec![3, 2], vec![2, 1]]);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::valid_arrangement(vec![vec![1, 2], vec![1, 3], vec![2, 1]]);
        assert_eq!(actual, vec![vec![1, 2], vec![2, 1], vec![1, 3]]);
    }
}

struct Solution;

fn main() {
    println!("Hello, world!");
}
