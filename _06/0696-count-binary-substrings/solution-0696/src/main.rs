impl Solution {
    #[allow(dead_code)]
    pub fn count_binary_substrings(chs: String) -> i32 {
        let mut res = 0;
        let bs = chs.as_bytes();
        let len = bs.len() as i32;
        for i in 0..len - 1 {
            for k in [b'0', b'1'] {
                let (mut l, mut r) = (i, i + 1);
                while l >= 0 && r < len && bs[l as usize] != bs[r as usize] && bs[l as usize] == k {
                    res += 1;
                    l -= 1;
                    r += 1;
                }
            }
        }
        res
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
        let actual = Solution::count_binary_substrings("00110011".into());
        assert_eq!(actual, 6);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::count_binary_substrings("10101".into());
        assert_eq!(actual, 4);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::count_binary_substrings("1010001".into());
        assert_eq!(actual, 4);
    }
}
