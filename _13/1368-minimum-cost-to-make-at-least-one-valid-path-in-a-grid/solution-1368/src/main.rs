use std::collections::VecDeque;

impl Solution {
    #[allow(dead_code)]
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        const RIGHT: i32 = 1;
        const LEFT: i32 = 2;
        const DOWN: i32 = 3;
        const UP: i32 = 4;

        let (m, n) = (grid.len() as i32, grid[0].len() as i32);

        let mut q: VecDeque<(Vec<(i32, i32)>, i32)> = VecDeque::new();
        q.push_back((vec![(0, 0)], 0));

        let mut res = vec![];

        while !q.is_empty() {
            if let Some(curr) = q.pop_front() {
                let last = *curr.0.last().unwrap();
                if last == (m - 1, n - 1) {
                    res.push(curr.1);
                    continue;
                }
                [
                    (last.0 - 1, last.1, UP),
                    (last.0 + 1, last.1, DOWN),
                    (last.0, last.1 - 1, LEFT),
                    (last.0, last.1 + 1, RIGHT),
                ]
                .iter()
                .filter(|(x, y, _)| *x >= 0 && *y >= 0 && *x < m && *y < n)
                .filter(|(x, y, _)| !curr.0.contains(&(*x, *y)))
                .for_each(|(x, y, d)| {
                    let mut next = curr.clone();
                    next.0.push((*x, *y));
                    next.1 = if grid[last.0 as usize][last.1 as usize] == *d {
                        curr.1
                    } else {
                        curr.1 + 1
                    };
                    q.push_back(next);
                });
            }
        }

        *res.iter().min().unwrap()
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
