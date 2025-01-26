impl Solution {
    #[allow(dead_code)]
    pub fn answer_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let mut nums = nums.clone();
        nums.sort();
        let nums = nums.iter().fold(vec![], |mut acc, &x| {
            acc.push(acc.last().unwrap_or(&0) + x);
            acc
        });
        queries.iter().fold(vec![], |mut acc, &x| {
            acc.push(
                1 + nums
                    .iter()
                    .rposition(|&y| y <= x)
                    .map(|x| x as i32)
                    .unwrap_or(-1),
            );
            acc
        })
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
        let actual = Solution::answer_queries(vec![4, 5, 2, 1], vec![3, 10, 21]);
        assert_eq!(actual, [2, 3, 4]);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::answer_queries(vec![2, 3, 4, 5], vec![1]);
        assert_eq!(actual, [0]);
    }
}
