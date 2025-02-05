impl Solution {
    #[allow(dead_code)]
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        if s1 == s2 {
            return true;
        }
        let (mut d, s1, s2) = (0, s1.as_bytes(), s2.as_bytes());
        let (mut f1, mut f2) = ([0; 28], [0; 28]);
        for i in 0..s1.len() {
            f1[(s1[i] - b'a') as usize] += 1;
            f2[(s2[i] - b'a') as usize] += 1;
            if s1[i] != s2[i] {
                d += 1;
            }
            if d == 3 {
                return false;
            }
        }
        (d == 2 || d == 0) && f1 == f2
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
        let actual = Solution::are_almost_equal("bank".into(), "kanb".into());
        assert!(actual);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::are_almost_equal("attack".into(), "defend".into());
        assert!(!actual);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::are_almost_equal("kelb".into(), "kelb".into());
        assert!(actual);
    }
}
