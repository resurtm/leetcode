impl Solution {
    #[allow(dead_code)]
    pub fn can_be_valid(s_inp: String, l_inp: String) -> bool {
        let check = |s: &[u8], l: &[u8], ch: u8| -> bool {
            let (mut lock, mut free) = (0, 0);
            for (id, &it) in s.iter().enumerate() {
                if l[id] == b'1' {
                    lock += if it == ch { 1 } else { -1 };
                } else {
                    free += 1;
                }
                if lock + free < 0 {
                    return false;
                }
            }
            lock <= free
        };
        let (s_tmp, l_tmp): (Vec<_>, Vec<_>) =
            (s_inp.bytes().rev().collect(), l_inp.bytes().rev().collect());
        s_inp.len() % 2 == 0
            && check(s_inp.as_bytes(), l_inp.as_bytes(), b'(')
            && check(&s_tmp, &l_tmp, b')')
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
        let actual = Solution::can_be_valid("))()))".into(), "010100".into());
        assert!(actual);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::can_be_valid("()()".into(), "0000".into());
        assert!(actual);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::can_be_valid(")".into(), "0".into());
        assert!(!actual);
    }

    #[test]
    fn test_case4() {
        let actual = Solution::can_be_valid("(".into(), "0".into());
        assert!(!actual);
    }

    #[test]
    fn test_case5() {
        let actual = Solution::can_be_valid("()".into(), "11".into());
        assert!(actual);
    }

    #[test]
    fn test_case6() {
        let actual = Solution::can_be_valid("()))".into(), "0010".into());
        assert!(actual);
    }

    #[test]
    fn test_case7() {
        let actual = Solution::can_be_valid("((())(((()()".into(), "100000111010".into());
        assert!(!actual);
    }

    #[test]
    fn test_case8() {
        let actual = Solution::can_be_valid(
            "((()(()()))()((()()))))()((()(()".into(),
            "10111100100101001110100010001001".into(),
        );
        assert!(actual);
    }
}
