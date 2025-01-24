impl Solution {
    #[allow(dead_code)]
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let len = graph.len();
        let mut st = vec![0u8; len];
        (0..len).fold(vec![], |mut acc, x| {
            if Self::dfs(x, &graph, &mut st) {
                acc.push(x as i32);
            }
            acc
        })
    }

    fn dfs(x: usize, graph: &[Vec<i32>], st: &mut [u8]) -> bool {
        if st[x] != 0 {
            return st[x] == 1;
        }
        st[x] = 2;
        for &y in graph[x].iter() {
            if !Self::dfs(y as usize, graph, st) {
                return false;
            }
        }
        st[x] = 1;
        true
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
        let actual = Solution::eventual_safe_nodes(vec![
            vec![1, 2],
            vec![2, 3],
            vec![5],
            vec![0],
            vec![5],
            vec![],
            vec![],
        ]);
        assert_eq!(actual, vec![2, 4, 5, 6]);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::eventual_safe_nodes(vec![
            vec![1, 2, 3, 4],
            vec![1, 2],
            vec![3, 4],
            vec![0, 4],
            vec![],
        ]);
        assert_eq!(actual, vec![4]);
    }

    #[test]
    fn test_case3() {
        let actual =
            Solution::eventual_safe_nodes(vec![vec![], vec![0, 2, 3, 4], vec![3], vec![4], vec![]]);
        assert_eq!(actual, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_case4() {
        let actual = Solution::eventual_safe_nodes(vec![
            vec![0],
            vec![2, 3, 4],
            vec![3, 4],
            vec![0, 4],
            vec![],
        ]);
        assert_eq!(actual, vec![4]);
    }
}
