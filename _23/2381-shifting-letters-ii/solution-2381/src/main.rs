impl Solution {
    pub fn shifting_letters(sr: String, shifts: Vec<Vec<i32>>) -> String {
        let mut s = sr.as_bytes().to_vec();
        for shift in shifts.iter() {
            if let [start, end, dir] = shift[0..3] {
                for i in start as usize..=end as usize {
                    let mut ch = s[i];
                    ch = if dir == 0 { ch - 1 } else { ch + 1 };
                    if ch > b'z' {
                        ch = b'a';
                    }
                    if ch < b'a' {
                        ch = b'z';
                    }
                    s[i] = ch;
                }
            }
        }
        String::from_utf8(s).unwrap()
    }
}

struct Solution;

fn main() {
    println!("nothing here");
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case1() {
        let actual = Solution::shifting_letters(
            "abc".into(),
            vec![vec![0, 1, 0], vec![1, 2, 1], vec![0, 2, 1]],
        );
        assert_eq!(actual, "ace");
    }

    #[test]
    fn test_case2() {
        let actual = Solution::shifting_letters("dztz".into(), vec![vec![0, 0, 0], vec![1, 1, 1]]);
        assert_eq!(actual, "catz");
    }
}
