use std::collections::HashSet;

impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        let mut blocked = HashSet::new();
        for coords in guards.iter() {
            blocked.insert([coords[0], coords[1]]);
        }
        for coords in walls.iter() {
            blocked.insert([coords[0], coords[1]]);
        }
        let mut res = blocked.clone();
        for coords in guards {
            let gx = coords[0];
            let gy = coords[1];

            for x in gx + 1..m {
                if blocked.contains(&[x, gy]) {
                    break;
                }
                res.insert([x, gy]);
            }
            for x in (0..gx).rev() {
                if blocked.contains(&[x, gy]) {
                    break;
                }
                res.insert([x, gy]);
            }
            for y in gy + 1..n {
                if blocked.contains(&[gx, y]) {
                    break;
                }
                res.insert([gx, y]);
            }
            for y in (0..gy).rev() {
                if blocked.contains(&[gx, y]) {
                    break;
                }
                res.insert([gx, y]);
            }
        }
        if false {
            for x in 0..m {
                for y in 0..n {
                    print!("{}", if res.contains(&[x, y]) { '#' } else { '.' });
                }
                println!("");
            }
        }
        m * n - res.len() as i32
    }
}

fn main() {
    println!("do nothing, see tests");
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case1() {
        let actual = Solution::count_unguarded(
            4,
            6,
            vec![vec![0, 0], vec![1, 1], vec![2, 3]],
            vec![vec![0, 1], vec![2, 2], vec![1, 4]],
        );
        assert_eq!(actual, 7);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::count_unguarded(
            3,
            3,
            vec![vec![1, 1]],
            vec![vec![0, 1], vec![1, 0], vec![2, 1], vec![1, 2]],
        );
        assert_eq!(actual, 4);
    }
}
