impl Solution {
    #[allow(dead_code)]
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let (w, h) = (mat.len(), mat[0].len());
        let pos = mat
            .iter()
            .enumerate()
            .fold(vec![(0usize, 0usize); arr.len()], |ai, (i, vs)| {
                vs.iter().enumerate().fold(ai, |mut aj, (j, v)| {
                    aj[*v as usize - 1] = (i, j);
                    aj
                })
            });
        let (mut is, mut js) = (vec![0usize; w], vec![0usize; h]);
        for (i, v) in arr.iter().enumerate() {
            let (m, n) = pos[*v as usize - 1];
            is[m] += 1;
            js[n] += 1;
            if is[m] == h || js[n] == w {
                return i as i32;
            }
        }
        0
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
        let actual = Solution::first_complete_index(vec![1, 3, 4, 2], vec![vec![1, 4], vec![2, 3]]);
        assert_eq!(actual, 2);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::first_complete_index(
            vec![2, 8, 7, 4, 1, 3, 5, 6, 9],
            vec![vec![3, 2, 5], vec![1, 4, 6], vec![8, 7, 9]],
        );
        assert_eq!(actual, 3);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::first_complete_index(
            vec![1, 4, 5, 2, 6, 3],
            vec![vec![4, 3, 5], vec![1, 2, 6]],
        );
        assert_eq!(actual, 1);
    }
}
