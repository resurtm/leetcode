use std::collections::VecDeque;

impl Solution {
    fn dfs(
        p: (usize, usize),
        grid: &[Vec<i32>],
        q: &mut VecDeque<(usize, usize)>,
        v: &mut [Vec<i32>],
        k: i32,
    ) {
        if q.contains(&p) || v[p.0][p.1] != i32::MAX {
            return;
        }
        q.push_back(p);
        v[p.0][p.1] = k;
        match grid[p.0][p.1] {
            4 => {
                if p.0 > 0 {
                    Self::dfs((p.0 - 1, p.1), grid, q, v, k);
                }
            }
            3 => {
                if p.0 < grid.len() - 1 {
                    Self::dfs((p.0 + 1, p.1), grid, q, v, k);
                }
            }
            2 => {
                if p.1 > 0 {
                    Self::dfs((p.0, p.1 - 1), grid, q, v, k);
                }
            }
            1 => {
                if p.1 < grid[0].len() - 1 {
                    Self::dfs((p.0, p.1 + 1), grid, q, v, k);
                }
            }
            _ => {}
        }
    }

    #[allow(dead_code)]
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let mut q: VecDeque<(usize, usize)> = VecDeque::new();
        let mut v = vec![vec![i32::MAX; grid[0].len()]; grid.len()];
        let mut k = 0;
        Self::dfs((0, 0), &grid, &mut q, &mut v, k);
        while !q.is_empty() {
            k += 1;
            let qc = std::mem::take(&mut q);
            qc.iter().for_each(|p| {
                if p.0 > 0 && !qc.contains(&(p.0 - 1, p.1)) {
                    Self::dfs((p.0 - 1, p.1), &grid, &mut q, &mut v, k);
                }
                if p.0 < grid.len() - 1 && !qc.contains(&(p.0 + 1, p.1)) {
                    Self::dfs((p.0 + 1, p.1), &grid, &mut q, &mut v, k);
                }
                if p.1 > 0 && !qc.contains(&(p.0, p.1 - 1)) {
                    Self::dfs((p.0, p.1 - 1), &grid, &mut q, &mut v, k);
                }
                if p.1 < grid[0].len() - 1 && !qc.contains(&(p.0, p.1 + 1)) {
                    Self::dfs((p.0, p.1 + 1), &grid, &mut q, &mut v, k);
                }
            });
        }
        *v.last().unwrap().last().unwrap()
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
        let actual = Solution::min_cost(vec![
            vec![1, 1, 1, 1],
            vec![2, 2, 2, 2],
            vec![1, 1, 1, 1],
            vec![2, 2, 2, 2],
        ]);
        assert_eq!(actual, 3);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::min_cost(vec![vec![1, 1, 3], vec![3, 2, 2], vec![1, 1, 4]]);
        assert_eq!(actual, 0);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::min_cost(vec![vec![1, 2], vec![4, 3]]);
        assert_eq!(actual, 1);
    }

    #[test]
    fn test_case4() {
        let actual = Solution::min_cost(vec![vec![1, 3], vec![1, 4]]);
        assert_eq!(actual, 0);
    }
}
