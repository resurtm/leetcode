impl Solution {
    #[allow(dead_code)]
    pub fn punishment_number(n: i32) -> i32 {
        let mut res = 0;
        for idx in 1..n + 1 {
            let mut acc = vec![];
            let sq = (idx * idx).to_string();
            if Self::check(idx, &sq, &mut acc, 0) {
                res += idx * idx;
            }
        }
        res
    }

    fn check<'a>(n: i32, ns: &'a str, acc: &mut Vec<&'a str>, pos: usize) -> bool {
        if n == 1 {
            return true;
        }
        if pos >= ns.len() {
            let sum = acc.iter().fold(0, |mut a, x| {
                a += x.parse::<i32>().unwrap();
                a
            });
            return sum == n;
        }
        for sz in (1..ns.len() - pos + 1).rev() {
            acc.push(&ns[pos..pos + sz]);
            if Self::check(n, ns, acc, pos + sz) {
                return true;
            }
            acc.pop();
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
        let actual = Solution::punishment_number(10);
        assert_eq!(actual, 182);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::punishment_number(37);
        assert_eq!(actual, 1478);
    }
}
