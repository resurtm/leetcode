impl Solution {
    pub fn max_score_sightseeing_pair(v: Vec<i32>) -> i32 {
        v.iter()
            .enumerate()
            .fold((0, 0), |(a, b), (ir, &it)| {
                let i = ir as i32;
                let x = if a < b + it - i { b + it - i } else { a };
                let y = if b < i + it { i + it } else { b };
                (x, y)
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case1() {
        let actual = Solution::max_score_sightseeing_pair(vec![8, 1, 5, 2, 6]);
        assert_eq!(actual, 11);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::max_score_sightseeing_pair(vec![1, 2]);
        assert_eq!(actual, 2);
    }
}

struct Solution;

fn main() {
    println!("nothing here");
}
