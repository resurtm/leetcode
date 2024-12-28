use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn shortest_bridge(grid_inp: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid_inp.clone();
        let w = grid.len() as i32;
        let h = grid[0].len() as i32;
        let mut res = i32::MAX;

        'outer: for i in 0..w {
            for j in 0..h {
                if grid[i as usize][j as usize] == 1 {
                    let island = Self::mark_first_island(&mut grid, w, h, i, j);
                    res = Self::move_to_second_island(&mut grid, &island, w, h);
                    break 'outer;
                }
            }
        }

        res
    }

    fn move_to_second_island(
        grid: &mut [Vec<i32>],
        island: &HashSet<(i32, i32)>,
        w: i32,
        h: i32,
    ) -> i32 {
        let mut q = VecDeque::new();
        island.iter().for_each(|it| q.push_front(*it));

        let mut num = 3;

        while !q.is_empty() {
            let mut q_next = VecDeque::new();

            while !q.is_empty() {
                if let Some((x, y)) = q.pop_front() {
                    let its = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
                    let its: Vec<_> = its
                        .iter()
                        .filter(|(m, n)| *m >= 0 && *n >= 0 && *m < w && *n < h)
                        .filter(|(m, n)| grid[*m as usize][*n as usize] <= 1)
                        .collect();
                    for (m, n) in its.iter() {
                        if grid[*m as usize][*n as usize] == 1 {
                            return num - 3;
                        }
                        q_next.push_back((*m, *n));
                        grid[*m as usize][*n as usize] = num;
                    }
                }
            }

            q = q_next;
            num += 1;
        }

        i32::MAX
    }

    fn mark_first_island(
        grid: &mut [Vec<i32>],
        w: i32,
        h: i32,
        i: i32,
        j: i32,
    ) -> HashSet<(i32, i32)> {
        let mut island = HashSet::new();
        island.insert((i, j));

        let mut q = VecDeque::new();
        q.push_front((i, j));

        grid[i as usize][j as usize] = 2;

        while !q.is_empty() {
            if let Some((x, y)) = q.pop_front() {
                let its = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
                let its: Vec<_> = its
                    .iter()
                    .filter(|(m, n)| *m >= 0 && *n >= 0 && *m < w && *n < h)
                    .filter(|(m, n)| grid[*m as usize][*n as usize] == 1)
                    .collect();
                its.iter().for_each(|(m, n)| {
                    island.insert((*m, *n));

                    q.push_back((*m, *n));

                    grid[*m as usize][*n as usize] = 2;
                });
            }
        }

        island
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
