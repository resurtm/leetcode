impl Solution {
    #[allow(dead_code)]
    pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
        let mut grid = grid.clone();
        let len = grid[0].len();

        let (mut front, mut back) = (vec![0i64; len], vec![0i64; len]);
        for i0 in 0..len {
            front[i0] = if i0 == 0 { 0 } else { front[i0 - 1] } + grid[0][i0] as i64;
            let i1 = len - i0 - 1;
            back[i1] = if i1 == len - 1 { 0 } else { back[i1 + 1] } + grid[1][i1] as i64;
        }

        let (mut imax, mut max) = (0, i64::MIN);
        for i in 0..len {
            let sum = front[i] + back[i];
            if max < sum {
                imax = i;
                max = sum;
            }
        }

        println!("{:?}", grid);
        println!("{} {}", imax, max);

        grid[0].splice(0..=imax, vec![0i32; imax + 1]);
        grid[1].splice(imax..len, vec![0i32; len - imax]);

        let (mut front, mut back) = (vec![0i64; len], vec![0i64; len]);
        for i0 in 0..len {
            front[i0] = if i0 == 0 { 0 } else { front[i0 - 1] } + grid[0][i0] as i64;
            let i1 = len - i0 - 1;
            back[i1] = if i1 == len - 1 { 0 } else { back[i1 + 1] } + grid[1][i1] as i64;
        }

        let (mut imax, mut max) = (0, i64::MIN);
        for i in 0..len {
            let sum = front[i] + back[i];
            if max < sum {
                imax = i;
                max = sum;
            }
        }

        println!("{:?}", grid);
        println!("{} {}", imax, max);

        max
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
