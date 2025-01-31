use std::collections::{HashMap, HashSet};

impl Solution {
    #[allow(dead_code)]
    pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
        let (w, h) = (grid.len() as i32, grid[0].len() as i32);
        let (mut islands, mut num_islands) = (vec![vec![(-1, -1); h as usize]; w as usize], 0);
        let mut viz: HashSet<(i32, i32)> = HashSet::new();
        let (mut has_zero, mut res) = (false, 0);
        for i in 0..w {
            for j in 0..h {
                if grid[i as usize][j as usize] == 0 {
                    has_zero = true;
                    continue;
                }
                if viz.contains(&(i, j)) {
                    continue;
                }
                let (island, island_size) = Self::bfs((i, j), &grid, w, h);
                for &(i, j) in island.iter() {
                    islands[i as usize][j as usize] = (num_islands, island_size);
                }
                num_islands += 1;
                viz.extend(island.iter());
                res = res.max(island_size);
            }
        }
        if !has_zero {
            return res;
        }
        for i in 0..w {
            for j in 0..h {
                if grid[i as usize][j as usize] != 0 {
                    continue;
                }
                let neighbors: HashMap<i32, i32> = [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)]
                    .iter()
                    .filter(|(m, n)| *m >= 0 && *n >= 0 && *m < w && *n < h)
                    .map(|(m, n)| islands[*m as usize][*n as usize])
                    .filter(|(_, n)| *n > 0)
                    .collect();
                res = res.max(
                    1 + neighbors.iter().fold(0, |mut acc, x| {
                        acc += x.1;
                        acc
                    }),
                );
            }
        }
        res
    }

    fn bfs(start: (i32, i32), grid: &[Vec<i32>], w: i32, h: i32) -> (HashSet<(i32, i32)>, i32) {
        let mut queue = vec![start];
        let mut viz = HashSet::from([start]);
        while !queue.is_empty() {
            if let Some(curr) = queue.pop() {
                let dirs: Vec<_> = [
                    (curr.0 - 1, curr.1),
                    (curr.0 + 1, curr.1),
                    (curr.0, curr.1 - 1),
                    (curr.0, curr.1 + 1),
                ]
                .iter()
                .filter(|(m, n)| *m >= 0 && *n >= 0 && *m < w && *n < h)
                .filter(|(m, n)| grid[*m as usize][*n as usize] == 1)
                .filter(|(m, n)| !viz.contains(&(*m, *n)))
                .map(|(m, n)| (*m, *n))
                .collect();
                for &dir in dirs.iter() {
                    queue.push(dir);
                    viz.insert(dir);
                }
            }
        }
        let size = viz.len() as i32;
        (viz, size)
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
        let actual = Solution::largest_island(vec![vec![1, 0], vec![0, 1]]);
        assert_eq!(actual, 3);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::largest_island(vec![vec![1, 1], vec![1, 0]]);
        assert_eq!(actual, 4);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::largest_island(vec![vec![1, 1], vec![1, 1]]);
        assert_eq!(actual, 4);
    }

    #[test]
    fn test_case4() {
        let actual = Solution::largest_island(vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 1, 1]]);
        assert_eq!(actual, 7);
    }

    #[test]
    fn test_case5() {
        let actual = Solution::largest_island(vec![vec![0, 0], vec![0, 0]]);
        assert_eq!(actual, 1);
    }

    #[test]
    fn test_case6() {
        let actual = Solution::largest_island(vec![vec![0, 0], vec![1, 1]]);
        assert_eq!(actual, 3);
    }

    #[test]
    fn test_case7() {
        let actual = Solution::largest_island(vec![vec![0, 1, 0], vec![1, 0, 1], vec![1, 0, 0]]);
        assert_eq!(actual, 5);
    }
}
