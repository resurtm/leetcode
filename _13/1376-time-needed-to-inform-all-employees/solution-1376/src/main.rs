impl Solution {
    #[allow(dead_code)]
    pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        let mut subs = vec![vec![0i32; 0]; n as usize];
        manager.iter().enumerate().for_each(|(i, x)| {
            if *x >= 0 {
                subs[*x as usize].push(i as i32);
            }
        });
        let mut r = 0;
        Self::dfs(
            head_id,
            inform_time[head_id as usize],
            &mut r,
            &subs,
            &inform_time,
        );
        r
    }

    fn dfs(curr: i32, m: i32, r: &mut i32, subs: &[Vec<i32>], inform_time: &[i32]) {
        *r = m.max(*r);
        subs[curr as usize].iter().for_each(|x| {
            Self::dfs(*x, inform_time[*x as usize] + m, r, subs, inform_time);
        });
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
        let actual = Solution::num_of_minutes(1, 0, vec![-1], vec![0]);
        assert_eq!(actual, 0);
    }

    #[test]
    fn test_case2() {
        let actual =
            Solution::num_of_minutes(6, 2, vec![2, 2, -1, 2, 2, 2], vec![0, 0, 1, 0, 0, 0]);
        assert_eq!(actual, 1);
    }

    #[test]
    fn test_case3() {
        let actual =
            Solution::num_of_minutes(6, 2, vec![2, 2, -1, 2, 2, 2], vec![0, 0, 1, 0, 0, 10]);
        assert_eq!(actual, 11);
    }
}
