use std::collections::{HashSet, VecDeque};

impl Solution {
    #[allow(dead_code)]
    pub fn trap_rain_water(g: Vec<Vec<i32>>) -> i32 {
        let top = g.iter().fold(0, |acc, gt| {
            acc.max(gt.iter().fold(0, |acc, x| acc.max(*x)))
        });
        let mut v = HashSet::new();
        g.iter().enumerate().for_each(|(i, gt)| {
            (0..gt.len()).for_each(|j| {
                Self::bfs1((i as i32, j as i32, top + 1), &g, &mut v);
            });
        });
        println!("{:?}", v);
        v.len() as i32
    }

    fn bfs1(pi: (i32, i32, i32), g: &[Vec<i32>], v: &mut HashSet<(i32, i32, i32)>) -> bool {
        let (m, n) = (g.len() as i32, g[0].len() as i32);
        let mut w = HashSet::new();
        w.insert(pi);
        let mut q = VecDeque::new();
        q.push_back(pi);
        while !q.is_empty() {
            if let Some(p) = q.pop_front() {
                Self::check(p, g, v);
                let dirs = [
                    (p.0 - 1, p.1, p.2),
                    (p.0 + 1, p.1, p.2),
                    (p.0, p.1 - 1, p.2),
                    (p.0, p.1 + 1, p.2),
                    (p.0, p.1, p.2 - 1),
                ];
                let dirs = dirs
                    .iter()
                    .filter(|(x, y, z)| *x >= 0 && *x < m && *y >= 0 && *y < n && *z >= 0)
                    .filter(|(x, y, z)| g[*x as usize][*y as usize] < *z)
                    .filter(|&t| !w.contains(t))
                    .map(|(x, y, z)| (*x, *y, *z))
                    .collect::<Vec<(i32, i32, i32)>>();
                dirs.iter().for_each(|&t| {
                    w.insert(t);
                    q.push_back(t);
                });
            }
        }
        true
    }

    fn check(pi: (i32, i32, i32), g: &[Vec<i32>], v: &mut HashSet<(i32, i32, i32)>) {
        let (m, n) = (g.len() as i32, g[0].len() as i32);
        for x in pi.0 + 1..=m {
            if x == m {
                return;
            }
            if g[x as usize][pi.1 as usize] >= pi.2 {
                break;
            }
        }
        for x in (-1..pi.0).rev() {
            if x == -1 {
                return;
            }
            if g[x as usize][pi.1 as usize] >= pi.2 {
                break;
            }
        }
        for y in pi.1 + 1..=n {
            if y == n {
                return;
            }
            if g[pi.0 as usize][y as usize] >= pi.2 {
                break;
            }
        }
        for y in (-1..pi.1).rev() {
            if y == -1 {
                return;
            }
            if g[pi.0 as usize][y as usize] >= pi.2 {
                break;
            }
        }
        v.insert(pi);
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
        let actual = Solution::trap_rain_water(vec![
            vec![1, 4, 3, 1, 3, 2],
            vec![3, 2, 1, 3, 2, 4],
            vec![2, 3, 3, 2, 3, 1],
        ]);
        assert_eq!(actual, 4);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::trap_rain_water(vec![
            vec![3, 3, 3, 3, 3],
            vec![3, 2, 2, 2, 3],
            vec![3, 2, 1, 2, 3],
            vec![3, 2, 2, 2, 3],
            vec![3, 3, 3, 3, 3],
        ]);
        assert_eq!(actual, 10);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::trap_rain_water(vec![
            vec![12, 13, 1, 12],
            vec![13, 4, 13, 12],
            vec![13, 8, 10, 12],
            vec![12, 13, 12, 12],
            vec![13, 13, 13, 13],
        ]);
        assert_eq!(actual, 14);
    }
}
