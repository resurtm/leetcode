impl Solution {
    #[allow(dead_code)]
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        if edges[0][0] == edges[1][0] || edges[0][0] == edges[1][1] {
            edges[0][0]
        } else {
            edges[0][1]
        }
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
        let actual = Solution::find_center(vec![vec![1, 2], vec![2, 3], vec![4, 2]]);
        assert_eq!(actual, 2);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::find_center(vec![vec![1, 2], vec![5, 1], vec![1, 3], vec![1, 4]]);
        assert_eq!(actual, 1);
    }
}
