impl Solution {
    pub fn pick_gifts(gifts_raw: Vec<i32>, k: i32) -> i64 {
        let mut gifts = gifts_raw.clone();
        for _ in 0..k {
            let mut imax = 0;
            let mut max = gifts[imax];
            for (idx, it) in gifts.iter().enumerate() {
                if max < *it {
                    imax = idx;
                    max = *it;
                }
            }
            gifts[imax] = f64::sqrt(max as f64) as i32;
        }
        gifts.iter().fold(0i64, |acc, x| acc + *x as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_case1() {
        let actual = Solution::pick_gifts(vec![25, 64, 9, 4, 100], 4);
        assert_eq!(actual, 29);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::pick_gifts(vec![1, 1, 1, 1], 4);
        assert_eq!(actual, 4);
    }
}

struct Solution;

fn main() {
    println!("Hello, world!");
}
