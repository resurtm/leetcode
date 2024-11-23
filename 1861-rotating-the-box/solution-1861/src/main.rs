impl Solution {
    pub fn rotate_the_box(bi: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let m = bi.len();
        let n = bi[0].len();
        let mut b: Vec<Vec<char>> = Vec::new();
        for _ in 0..n {
            let mut t: Vec<char> = Vec::new();
            for _ in 0..m {
                t.push(' ');
            }
            b.push(t);
        }
        for i in 0..m {
            for j in 0..n {
                b[j][i] = bi[m - i - 1][j];
            }
        }
        for i in 0..m {
            for j in (0..n - 1).rev() {
                if b[j][i] != '#' {
                    continue;
                }
                for k in j..n - 1 {
                    if b[k + 1][i] != '.' {
                        break;
                    }
                    let t = b[k + 1][i];
                    b[k + 1][i] = b[k][i];
                    b[k][i] = t;
                }
            }
        }
        b
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case1() {
        let actual = Solution::rotate_the_box(vec![vec!['#', '.', '#']]);
        assert_eq!(actual, vec![vec!['.'], vec!['#'], vec!['#']]);
    }

    #[test]
    fn test_case2() {
        let actual =
            Solution::rotate_the_box(vec![vec!['#', '.', '*', '.'], vec!['#', '#', '*', '.']]);
        assert_eq!(
            actual,
            vec![
                vec!['#', '.'],
                vec!['#', '#'],
                vec!['*', '*'],
                vec!['.', '.']
            ]
        );
    }

    #[test]
    fn test_case3() {
        let actual = Solution::rotate_the_box(vec![
            vec!['#', '#', '*', '.', '*', '.'],
            vec!['#', '#', '#', '*', '.', '.'],
            vec!['#', '#', '#', '.', '#', '.'],
        ]);
        assert_eq!(
            actual,
            vec![
                vec!['.', '#', '#'],
                vec!['.', '#', '#'],
                vec!['#', '#', '*'],
                vec!['#', '*', '.'],
                vec!['#', '.', '*'],
                vec!['#', '.', '.']
            ]
        );
    }
}

struct Solution;

fn main() {
    println!("Hello, world!");
}
