use std::collections::HashSet;

impl Solution {
    #[allow(dead_code)]
    pub fn check_if_prerequisite(
        num_courses: i32,
        prerequisites: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let links =
            prerequisites
                .iter()
                .fold(vec![vec![]; num_courses as usize], |mut acc, link| {
                    let (src, dst) = (link[0], link[1]);
                    acc[dst as usize].push(src);
                    acc
                });

        let (mut dirs, mut acc) = (vec![HashSet::new(); num_courses as usize], vec![]);
        for course in 0..num_courses {
            Self::dfs(course, &mut acc, &links, &mut dirs);
        }

        queries
            .iter()
            .map(|link| {
                let (src, dst) = (link[0], link[1]);
                dirs[src as usize].contains(&dst)
            })
            .collect()
    }

    fn dfs(curr: i32, acc: &mut Vec<i32>, links: &[Vec<i32>], dirs: &mut [HashSet<i32>]) {
        dirs[curr as usize].extend(acc.clone());
        acc.push(curr);
        for &link in links[curr as usize].iter() {
            Self::dfs(link, acc, links, dirs);
        }
        acc.pop();
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
