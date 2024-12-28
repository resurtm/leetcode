use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn shortest_bridge(grid: Vec<Vec<i32>>) -> i32 {
        let w = grid.len() as i32;
        let h = grid[0].len() as i32;

        let mut viz = HashSet::new();
        let mut island0 = HashSet::new();
        let mut island1 = HashSet::new();

        for i in 0..w {
            for j in 0..h {
                if grid[i as usize][j as usize] == 0 || viz.contains(&(i, j)) {
                    continue;
                }
                let island = Self::find_island(&grid, i, j);
                island.iter().for_each(|(x, y)| {
                    viz.insert((*x, *y));
                });
                if island0.is_empty() {
                    island0 = island.clone();
                } else if island1.is_empty() {
                    island1 = island.clone();
                }
            }
        }

        let mut dist = i32::MAX;
        for (i, j) in island0.iter() {
            Self::check_dist(w, h, &island0, &island1, *i, *j, &mut dist);
        }
        dist
    }

    fn check_dist(
        w: i32,
        h: i32,
        island0: &HashSet<(i32, i32)>,
        island1: &HashSet<(i32, i32)>,
        i: i32,
        j: i32,
        dist: &mut i32,
    ) {
        let mut viz = HashSet::new();

        let mut q = VecDeque::new();
        q.push_front((i, j, 0));
        viz.insert((i, j));

        while !q.is_empty() {
            if let Some((x, y, d)) = q.pop_front() {
                let its = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
                let its: Vec<_> = its
                    .iter()
                    .filter(|(m, n)| *m >= 0 && *n >= 0 && *m < w && *n < h)
                    .filter(|&it| !island0.contains(it))
                    .filter(|&it| !viz.contains(it))
                    .collect();
                its.iter().for_each(|(m, n)| {
                    if island1.contains(&(*m, *n)) {
                        if *dist > d {
                            *dist = d;
                        }
                    } else {
                        q.push_back((*m, *n, d + 1));
                        viz.insert((*m, *n));
                    }
                });
            }
        }
    }

    fn find_island(grid: &[Vec<i32>], i: i32, j: i32) -> HashSet<(i32, i32)> {
        let w = grid.len() as i32;
        let h = grid[0].len() as i32;

        let mut viz = HashSet::new();

        let mut q = VecDeque::new();
        q.push_front((i, j));
        viz.insert((i, j));

        while !q.is_empty() {
            if let Some((x, y)) = q.pop_front() {
                let its = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
                let its: Vec<_> = its
                    .iter()
                    .filter(|(m, n)| *m >= 0 && *n >= 0 && *m < w && *n < h)
                    .filter(|(m, n)| grid[*m as usize][*n as usize] == 1)
                    .filter(|&it| !viz.contains(it))
                    .collect();
                its.iter().for_each(|(m, n)| {
                    q.push_back((*m, *n));
                    viz.insert((*m, *n));
                });
            }
        }

        viz
    }
}

struct Solution;

fn main() {
    println!("nothing here");
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case1() {
        let actual = Solution::shortest_bridge(vec![vec![0, 1], vec![1, 0]]);
        assert_eq!(actual, 1);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::shortest_bridge(vec![vec![0, 1, 0], vec![0, 0, 0], vec![0, 0, 1]]);
        assert_eq!(actual, 2);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::shortest_bridge(vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 1],
            vec![1, 0, 1, 0, 1],
            vec![1, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1],
        ]);
        assert_eq!(actual, 1);
    }
}
