impl Solution {
    #[allow(dead_code)]
    pub fn check_if_prerequisite(
        num_courses: i32,
        prerequisites: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let mut reach = vec![vec![false; num_courses as usize]; num_courses as usize];
        for prerequisite in prerequisites.iter() {
            let (a, b) = (prerequisite[0] as usize, prerequisite[1] as usize);
            reach[a][b] = true;
        }
        for k in 0..num_courses as usize {
            for i in 0..num_courses as usize {
                for j in 0..num_courses as usize {
                    reach[i][j] = reach[i][j] || reach[i][k] && reach[k][j];
                }
            }
        }
        queries
            .iter()
            .map(|link| {
                let (src, dst) = (link[0] as usize, link[1] as usize);
                reach[src][dst]
            })
            .collect()
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
        let actual =
            Solution::check_if_prerequisite(2, vec![vec![1, 0]], vec![vec![0, 1], vec![1, 0]]);
        assert_eq!(actual, [false, true]);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::check_if_prerequisite(2, vec![], vec![vec![1, 0], vec![0, 1]]);
        assert_eq!(actual, [false, false]);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::check_if_prerequisite(
            3,
            vec![vec![1, 2], vec![1, 0], vec![2, 0]],
            vec![vec![1, 0], vec![1, 2]],
        );
        assert_eq!(actual, [true, true]);
    }

    #[test]
    fn test_case4() {
        let actual = Solution::check_if_prerequisite(
            5,
            vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]],
            vec![vec![0, 4], vec![4, 0], vec![1, 3], vec![3, 0]],
        );
        assert_eq!(actual, [true, false, true, false]);
    }
}
