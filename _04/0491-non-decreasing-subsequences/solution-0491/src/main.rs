use std::collections::HashSet;

impl Solution {
    #[allow(dead_code)]
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = HashSet::new();
        for (pos, &num) in nums.iter().enumerate() {
            Self::backtrack(pos, vec![vec![num]], &mut res, &nums);
        }
        res.into_iter().collect()
    }

    fn backtrack(pos: usize, acc: Vec<Vec<i32>>, res: &mut HashSet<Vec<i32>>, nums: &Vec<i32>) {
        for x in acc.iter() {
            if x.len() >= 2 {
                res.insert(x.clone());
            }
        }
        if pos == nums.len() - 1 {
            return;
        }
        let mut to_add: Vec<Vec<i32>> = vec![];
        for x in acc.iter() {
            let num = *nums.get(pos + 1).unwrap();
            if *x.last().unwrap() <= num {
                to_add.push(x.iter().chain([num].iter()).cloned().collect());
            }
        }
        Self::backtrack(
            pos + 1,
            acc.iter().chain(to_add.iter()).cloned().collect(),
            res,
            nums,
        );
    }
}

struct Solution {}

fn main() {
    println!("nothing here");
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use std::collections::HashSet;

    #[test]
    fn test_case1() {
        let actual = Solution::find_subsequences(vec![4, 6, 7, 7]);
        let actual: HashSet<Vec<i32>> = HashSet::from_iter(actual.iter().cloned());
        let expected = HashSet::from_iter(
            [
                vec![4, 6],
                vec![4, 6, 7],
                vec![4, 6, 7, 7],
                vec![4, 7],
                vec![4, 7, 7],
                vec![6, 7],
                vec![6, 7, 7],
                vec![7, 7],
            ]
            .iter()
            .cloned(),
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::find_subsequences(vec![4, 4, 3, 2, 1]);
        let actual: HashSet<Vec<i32>> = HashSet::from_iter(actual.iter().cloned());
        let expected = HashSet::from_iter([vec![4, 4]].iter().cloned());
        assert_eq!(actual, expected);
    }
}
