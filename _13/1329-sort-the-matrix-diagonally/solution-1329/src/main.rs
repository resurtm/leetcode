impl Solution {
    #[allow(dead_code)]
    pub fn diagonal_sort(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut mat = mat.clone();
        let (w, h) = (mat.len(), mat[0].len());
        for i in 0..w {
            let mut items = vec![];
            for k in 0..h.min(w - i) {
                items.push(mat[i + k][k]);
            }
            items.sort();
            for k in 0..h.min(w - i) {
                mat[i + k][k] = items[k];
            }
        }
        for j in 1..h {
            let mut items = vec![];
            for k in 0..w.min(h - j) {
                items.push(mat[k][j + k]);
            }
            items.sort();
            for k in 0..w.min(h - j) {
                mat[k][j + k] = items[k];
            }
        }
        mat
    }
}

struct Solution {}

fn main() {
    println!("nothing here");
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case1() {
        let actual =
            Solution::diagonal_sort(vec![vec![3, 3, 1, 1], vec![2, 2, 1, 2], vec![1, 1, 1, 2]]);
        assert_eq!(actual, [[1, 1, 1, 1], [1, 2, 2, 2], [1, 2, 3, 3]]);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::diagonal_sort(vec![
            vec![11, 25, 66, 1, 69, 7],
            vec![23, 55, 17, 45, 15, 52],
            vec![75, 31, 36, 44, 58, 8],
            vec![22, 27, 33, 25, 68, 4],
            vec![84, 28, 14, 11, 5, 50],
        ]);
        assert_eq!(
            actual,
            [
                [5, 17, 4, 1, 52, 7],
                [11, 11, 25, 45, 8, 69],
                [14, 23, 25, 44, 58, 15],
                [22, 27, 31, 36, 50, 66],
                [84, 28, 75, 33, 55, 68]
            ]
        );
    }
}
