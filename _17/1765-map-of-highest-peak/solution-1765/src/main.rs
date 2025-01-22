impl Solution {
    #[allow(dead_code)]
    pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (is_water.len(), is_water[0].len());
        let (mi, ni) = (m as i32, n as i32);
        let (mut c, mut q, mut r) = (1, vec![], vec![vec![-1; n]; m]);
        for (i, vs) in is_water.iter().enumerate() {
            for (j, v) in vs.iter().enumerate() {
                if *v == 1 {
                    q.push((i as i32, j as i32));
                    r[i][j] = 0;
                }
            }
        }
        while !q.is_empty() {
            for (i, j) in std::mem::take(&mut q).iter() {
                let dirs: Vec<(i32, i32)> =
                    [(*i - 1, *j), (*i + 1, *j), (*i, *j - 1), (*i, *j + 1)]
                        .iter()
                        .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < mi && *y < ni)
                        .filter(|(x, y)| r[*x as usize][*y as usize] == -1)
                        .map(|(x, y)| (*x, *y))
                        .collect();
                dirs.iter().for_each(|(x, y)| {
                    q.push((*x, *y));
                    r[*x as usize][*y as usize] = c;
                });
            }
            c += 1;
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
        let actual = Solution::highest_peak(vec![vec![0, 1], vec![0, 0]]);
        assert_eq!(actual, vec![vec![1, 0], vec![2, 1]]);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::highest_peak(vec![vec![0, 0, 1], vec![1, 0, 0], vec![0, 0, 0]]);
        assert_eq!(actual, vec![vec![1, 1, 0], vec![0, 1, 1], vec![1, 2, 2]]);
    }
}
