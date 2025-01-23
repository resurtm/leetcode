use std::collections::{HashSet, VecDeque};

impl Solution {
    #[allow(dead_code)]
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let (si, sj) = (grid.len() as i32, grid[0].len() as i32);
        let (mut r, mut v) = (0, HashSet::new());
        for (x, its) in grid.iter().enumerate() {
            for (y, &it) in its.iter().enumerate() {
                let (x, y) = (x as i32, y as i32);
                if it == 0 || v.contains(&(x, y)) {
                    continue;
                }
                let (mut w, mut q) = (HashSet::new(), VecDeque::from([(x, y)]));
                while !q.is_empty() {
                    if let Some((i, j)) = q.pop_front() {
                        for t in 0..si {
                            if grid[t as usize][j as usize] == 1
                                && !v.contains(&(t, j))
                                && !w.contains(&(t, j))
                            {
                                w.insert((t, j));
                                q.push_back((t, j));
                            }
                        }
                        for t in 0..sj {
                            if grid[i as usize][t as usize] == 1
                                && !v.contains(&(i, t))
                                && !w.contains(&(i, t))
                            {
                                w.insert((i, t));
                                q.push_back((i, t));
                            }
                        }
                    }
                }
                if w.len() > 1 {
                    r += w.len() as i32;
                }
                v.extend(w);
            }
        }
        r
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
        let actual = Solution::count_servers(vec![vec![1, 0], vec![0, 1]]);
        assert_eq!(actual, 0);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::count_servers(vec![vec![1, 0], vec![1, 1]]);
        assert_eq!(actual, 3);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::count_servers(vec![
            vec![1, 1, 0, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 1],
        ]);
        assert_eq!(actual, 4);
    }

    #[test]
    fn test_case4() {
        let actual = Solution::count_servers(vec![
            vec![1, 0, 0, 1, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 1, 0],
        ]);
        assert_eq!(actual, 3);
    }
}
