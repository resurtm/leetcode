use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

impl Solution {
    #[allow(dead_code)]
    pub fn trap_rain_water(g: Vec<Vec<i32>>) -> i32 {
        let (m, n, mi, ni) = (g.len(), g[0].len(), g.len() as i32, g[0].len() as i32);
        let mut v = HashSet::new();
        let mut h = BinaryHeap::new();
        (0..m).for_each(|i| {
            (0..n).for_each(|j| {
                if i == 0 || j == 0 || i == m - 1 || j == n - 1 {
                    v.insert((i as i32, j as i32));
                    h.push(Reverse((g[i][j], i as i32, j as i32)));
                }
            })
        });
        let mut r = 0;
        while !h.is_empty() {
            if let Some(it) = h.pop() {
                let (k, i, j) = it.0;
                let ds = [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)]
                    .iter()
                    .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < mi && *y < ni)
                    .filter(|&t| !v.contains(t))
                    .map(|(x, y)| (*x, *y))
                    .collect::<Vec<(i32, i32)>>();
                ds.iter().for_each(|(x, y)| {
                    let gh = g[*x as usize][*y as usize];
                    r += 0.max(k - gh);
                    v.insert((*x, *y));
                    h.push(Reverse((k.max(gh), *x, *y)));
                });
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
