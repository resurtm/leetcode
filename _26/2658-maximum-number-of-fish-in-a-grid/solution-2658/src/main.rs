use std::collections::HashSet;

impl Solution {
    #[allow(dead_code)]
    pub fn find_max_fish(grid: Vec<Vec<i32>>) -> i32 {
        let (mut res, mut viz) = (0, HashSet::new());
        for (i, its) in grid.iter().enumerate() {
            for (j, &it) in its.iter().enumerate() {
                let (i, j) = (i as i32, j as i32);
                if it != 0 && !viz.contains(&(i, j)) {
                    res = res.max(Self::bfs((i, j), &mut viz, &grid));
                }
            }
        }
        res
    }

    fn bfs(pos: (i32, i32), viz: &mut HashSet<(i32, i32)>, grid: &[Vec<i32>]) -> i32 {
        let (m, n) = (grid.len() as i32, grid[0].len() as i32);
        let (mut res, mut queue) = (grid[pos.0 as usize][pos.1 as usize], vec![pos]);
        viz.insert(pos);
        while !queue.is_empty() {
            if let Some(curr) = queue.pop() {
                let (x, y) = (curr.0, curr.1);
                let dirs: Vec<(i32, i32)> = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                    .iter()
                    .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < m && *y < n)
                    .filter(|(x, y)| grid[*x as usize][*y as usize] > 0)
                    .filter(|(x, y)| !viz.contains(&(*x, *y)))
                    .map(|(x, y)| (*x, *y))
                    .collect();
                dirs.iter().for_each(|&x| {
                    res += grid[x.0 as usize][x.1 as usize];
                    queue.push(x);
                    viz.insert(x);
                });
            }
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
        let actual = Solution::find_max_fish(vec![
            vec![0, 2, 1, 0],
            vec![4, 0, 0, 3],
            vec![1, 0, 0, 4],
            vec![0, 3, 2, 0],
        ]);
        assert_eq!(actual, 7);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::find_max_fish(vec![
            vec![1, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 1],
        ]);
        assert_eq!(actual, 1);
    }
}
