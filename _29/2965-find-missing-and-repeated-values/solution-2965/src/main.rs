use std::collections::HashSet;

impl Solution {
    #[allow(dead_code)]
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut vis = HashSet::new();
        let (mut a, mut b, mut max) = (-1, -1, -1);
        for items in grid.iter() {
            for &item in items.iter() {
                if vis.contains(&item) {
                    a = item;
                }
                vis.insert(item);
                max = max.max(item);
            }
        }
        for item in 1..=max {
            if !vis.contains(&item) {
                b = item;
                break;
            }
        }
        if b == -1 {
            b = max + 1;
        }
        vec![a, b]
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
        let actual = Solution::find_missing_and_repeated_values(vec![vec![1, 3], vec![2, 2]]);
        assert_eq!(actual, [2, 4]);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::find_missing_and_repeated_values(vec![
            vec![9, 1, 7],
            vec![8, 9, 2],
            vec![3, 4, 6],
        ]);
        assert_eq!(actual, [9, 5]);
    }
}
