use std::collections::HashSet;

impl Solution {
    #[allow(dead_code)]
    pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {
        let len = n * 2 - 1;
        let mut res = vec![0; len as usize];
        let mut used = HashSet::new();
        Self::backtrack(n, 0, n * 2 - 1, &mut res, &mut used);
        res
    }

    fn backtrack(n: i32, pos: i32, len: i32, res: &mut Vec<i32>, used: &mut HashSet<i32>) -> bool {
        if pos == len {
            return true;
        }
        if res[pos as usize] != 0 {
            return Self::backtrack(n, pos + 1, len, res, used);
        }
        for x in (1..n + 1).rev() {
            if used.contains(&x) {
                continue;
            }
            let next = if x > 1 { pos + x } else { pos };
            if next >= len || res[next as usize] != 0 {
                continue;
            }
            res[pos as usize] = x;
            res[next as usize] = x;
            used.insert(x);
            if Self::backtrack(n, pos, len, res, used) {
                return true;
            }
            res[pos as usize] = 0;
            res[next as usize] = 0;
            used.remove(&x);
        }
        false
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
        let actual = Solution::construct_distanced_sequence(3);
        assert_eq!(actual, [3, 1, 2, 3, 2]);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::construct_distanced_sequence(5);
        assert_eq!(actual, [5, 3, 1, 4, 3, 5, 2, 4, 2]);
    }
}
