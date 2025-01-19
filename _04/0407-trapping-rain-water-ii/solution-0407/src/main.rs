use std::collections::{HashSet, VecDeque};

impl Solution {
    #[allow(dead_code)]
    pub fn trap_rain_water(g: Vec<Vec<i32>>) -> i32 {
        let mut v = HashSet::new();
        g.iter().enumerate().for_each(|(i, gt)| {
            gt.iter().enumerate().for_each(|(j, ki)| {
                let mut k = *ki;
                while Self::bfs((i as i32, j as i32, k + 1), &g, &mut v) {
                    k += 1;
                }
            });
        });
        v.len() as i32
    }

    fn bfs(pi: (i32, i32, i32), g: &[Vec<i32>], v: &mut HashSet<(i32, i32, i32)>) -> bool {
        let (m, n) = (g.len() as i32, g[0].len() as i32);
        let mut w = HashSet::new();
        w.insert(pi);
        let mut q = VecDeque::new();
        q.push_back(pi);
        while !q.is_empty() {
            if let Some(p) = q.pop_front() {
                let dirs = [
                    (p.0 - 1, p.1, p.2),
                    (p.0 + 1, p.1, p.2),
                    (p.0, p.1 - 1, p.2),
                    (p.0, p.1 + 1, p.2),
                    (p.0, p.1, p.2 - 1),
                ];
                if dirs
                    .iter()
                    .any(|(x, y, z)| *x < 0 || *x >= m || *y < 0 || *y >= n || *z < 0)
                {
                    return false;
                }
                let dirs = dirs
                    .iter()
                    .filter(|(x, y, z)| g[*x as usize][*y as usize] < *z)
                    .filter(|&t| !v.contains(t) && !w.contains(t))
                    .map(|(x, y, z)| (*x, *y, *z))
                    .collect::<Vec<(i32, i32, i32)>>();
                dirs.iter().for_each(|&t| {
                    w.insert(t);
                    q.push_back(t);
                });
            }
        }
        v.extend(w);
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
