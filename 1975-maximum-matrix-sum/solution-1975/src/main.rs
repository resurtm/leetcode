impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let mut mat = matrix.clone();
        Self::print_mat(&mat);

        // step 1 - for every row, make only a single negative element
        for j in 0..mat.len() {
            if mat[j].iter().all(|&x| x >= 0)
                || mat[j].iter().filter(|&&x| x < 0).count() == 1 && mat[j][0] < 0
            {
                continue;
            }
            for i in (0..mat[j].len()).rev() {
                if i > 0 && mat[j][i] < 0 {
                    mat[j][i] = -mat[j][i];
                    mat[j][i - 1] = -mat[j][i - 1];
                }
            }
        }

        // step 2 - for the first row, make only a single negative element
        for j in (0..mat.len()).rev() {
            if j > 0 && mat[j][0] < 0 {
                mat[j][0] = -mat[j][0];
                mat[j - 1][0] = -mat[j - 1][0];
            }
        }

        // step 3 - swap signs between min and negative
        let mut negp: Option<(usize, usize)> = None;
        let mut minp: Option<(usize, usize)> = None;
        let mut min: Option<i32> = None;
        for (j, n) in mat.iter().enumerate() {
            for (i, &m) in n.iter().enumerate() {
                if m < 0 {
                    negp = Some((j, i));
                }
                if let Some(x) = min {
                    if m.abs() < x {
                        minp = Some((j, i));
                        min = Some(m.abs());
                    }
                } else {
                    minp = Some((j, i));
                    min = Some(m.abs());
                }
            }
        }
        if let (Some((x0, y0)), Some((x1, y1))) = (negp, minp) {
            mat[x0][y0] = -mat[x0][y0];
            mat[x1][y1] = -mat[x1][y1];
        }

        Self::print_mat(&mat);
        mat.iter().fold(0, |acc0, x0| {
            acc0 + x0.iter().fold(0, |acc1, &x1| acc1 + x1) as i64
        })
    }

    fn print_mat(mat: &Vec<Vec<i32>>) {
        return;
        println!("{}", "-".repeat(10));
        for n in mat.iter() {
            for m in n.iter() {
                print!("{: >7}", m);
            }
            println!();
        }
        println!("{}", "-".repeat(10));
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case1() {
        let actual = Solution::max_matrix_sum(vec![vec![1, -1], vec![-1, 1]]);
        assert_eq!(actual, 4);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::max_matrix_sum(vec![vec![1, 2, 3], vec![-1, -2, -3], vec![1, 2, 3]]);
        assert_eq!(actual, 16);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::max_matrix_sum(vec![vec![2, 9, 3], vec![5, 4, -4], vec![1, 7, 1]]);
        assert_eq!(actual, 34);
    }

    #[test]
    fn test_case4() {
        let actual = Solution::max_matrix_sum(vec![
            vec![-8, -9, -10, 1],
            vec![-5, -7, -10, -3],
            vec![-8, -4, -2, -6],
            vec![2, -1, 8, -3],
        ]);
        assert_eq!(actual, 85);
    }

    #[test]
    fn test_case5() {
        let actual = Solution::max_matrix_sum(vec![
            vec![-56261, -15288, -59083, -14357, -15751],
            vec![-48494, -32094, -87818, -33356, -16991],
            vec![-72395, -48735, -21856, -30471, -80400],
            vec![-33852, -17577, -88317, -59620, -94630],
            vec![-69472, -40030, -26429, -69577, -31498],
        ]);
        assert_eq!(actual, 1135638);
    }
}

struct Solution;

fn main() {
    println!("Hello, world!");
}
