impl Solution {
    #[allow(dead_code)]
    pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
        let (mut t, mut b, mut r) = (
            grid[0].iter().fold(0i64, |acc, x| acc + *x as i64),
            0,
            i64::MAX,
        );
        for (i, j) in grid[0].iter().zip(grid[1].iter()) {
            t -= *i as i64;
            r = r.min(t.max(b));
            b += *j as i64;
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
        let actual = Solution::grid_game(vec![vec![2, 5, 4], vec![1, 5, 1]]);
        assert_eq!(actual, 4);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::grid_game(vec![vec![3, 3, 1], vec![8, 5, 2]]);
        assert_eq!(actual, 4);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::grid_game(vec![vec![1, 3, 1, 15], vec![1, 3, 3, 1]]);
        assert_eq!(actual, 7);
    }

    #[test]
    fn test_case4() {
        let actual = Solution::grid_game(vec![
            vec![20, 3, 20, 17, 2, 12, 15, 17, 4, 15],
            vec![20, 10, 13, 14, 15, 5, 2, 3, 14, 3],
        ]);
        assert_eq!(actual, 63);
    }
}
