impl Solution {
    fn check(days: &Vec<i32>, costs: &Vec<i32>, idx: i32, cache: &mut Vec<i32>) -> i32 {
        let len = days.len() as i32;
        if idx >= len {
            return 0;
        }
        if cache[idx as usize] != i32::MAX {
            return cache[idx as usize];
        }
        // 1 day pass
        let cost_day = costs[0] + Self::check(days, costs, idx + 1, cache);
        // 7 day pass
        let cost_week = {
            let mut next_idx = idx;
            while next_idx < len && days[next_idx as usize] - 7 < days[idx as usize] {
                next_idx += 1;
            }
            if idx != next_idx {
                costs[1] + Self::check(days, costs, next_idx, cache)
            } else {
                i32::MAX
            }
        };
        // 30 day pass
        let cost_month = {
            let mut next_idx = idx;
            while next_idx < len && days[next_idx as usize] - 30 < days[idx as usize] {
                next_idx += 1;
            }
            if idx != next_idx {
                costs[2] + Self::check(days, costs, next_idx, cache)
            } else {
                i32::MAX
            }
        };
        let ret = *[cost_day, cost_week, cost_month].iter().min().unwrap();
        cache[idx as usize] = ret;
        ret
    }

    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let mut cache = vec![];
        cache.resize(365 + 1 + 1, i32::MAX);
        Self::check(&days, &costs, 0, &mut cache)
    }
}

struct Solution;

fn main() {
    let res = Solution::mincost_tickets(vec![1, 4, 6, 7, 8, 20], vec![2, 7, 15]);
    println!("{}", res);
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case1() {
        let actual = Solution::mincost_tickets(vec![1, 4, 6, 7, 8, 20], vec![2, 7, 15]);
        assert_eq!(actual, 11);
    }

    #[test]
    fn test_case2() {
        let actual =
            Solution::mincost_tickets(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31], vec![2, 7, 15]);
        assert_eq!(actual, 17);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::mincost_tickets(
            vec![
                1, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 20, 21, 24, 25, 27, 28,
                29, 30, 31, 34, 37, 38, 39, 41, 43, 44, 45, 47, 48, 49, 54, 57, 60, 62, 63, 66, 69,
                70, 72, 74, 76, 78, 80, 81, 82, 83, 84, 85, 88, 89, 91, 93, 94, 97, 99,
            ],
            vec![9, 38, 134],
        );
        assert_eq!(actual, 423);
    }
}
