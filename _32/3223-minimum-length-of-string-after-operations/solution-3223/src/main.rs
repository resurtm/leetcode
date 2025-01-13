impl Solution {
    #[allow(dead_code)]
    pub fn minimum_length(si: String) -> i32 {
        let mut s = si.into_bytes();
        loop {
            let mut removed = false;
            let mut idx = -1;
            loop {
                idx += 1;
                let id = idx as usize;
                if id == s.len() {
                    break;
                }
                if id == 0 || id == s.len() - 1 {
                    continue;
                }
                let it = s[id];

                let mut l = usize::MAX;
                for idl in 0..id {
                    if s[idl] == it {
                        l = idl;
                        break;
                    }
                }
                if l == usize::MAX {
                    continue;
                }

                let mut r = usize::MAX;
                for idr in (id + 1..s.len()).rev() {
                    if s[idr] == it {
                        r = idr;
                        break;
                    }
                }
                if r == usize::MAX {
                    continue;
                }

                removed = true;
                idx -= 2;
                s.remove(l);
                s.remove(r - 1);
            }
            if !removed {
                break;
            }
        }
        s.len() as i32
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
        let actual = Solution::minimum_length("abaacbcbb".into());
        assert_eq!(actual, 5);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::minimum_length("aa".into());
        assert_eq!(actual, 2);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::minimum_length("lyqkwhyy".into());
        assert_eq!(actual, 6);
    }
}
