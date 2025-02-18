use std::collections::VecDeque;

impl Solution {
    #[allow(dead_code)]
    pub fn smallest_number(pattern: String) -> String {
        let mut st = VecDeque::new();
        let mut used = [false; 9];
        for nxt in 1..=9 {
            st.push_back(nxt);
            used[nxt - 1] = true;
            if let Some(ret) = Self::backtrack(pattern.as_bytes(), 0, nxt, &mut st, &mut used) {
                return ret;
            }
            st.pop_back();
            used[nxt - 1] = false;
        }
        String::new()
    }

    fn backtrack(
        pattern: &[u8],
        pos: usize,
        next: usize,
        st: &mut VecDeque<usize>,
        used: &mut [bool; 9],
    ) -> Option<String> {
        if pos == pattern.len() {
            // println!("{:?} {:?} {:?} {:?}", pos, next, st, used);
            return Some(st.iter().map(|&x| x.to_string()).collect());
        }
        match pattern[pos] {
            b'D' => {
                let mut nxt = next - 1;
                while nxt != 0 {
                    if !used[nxt - 1] {
                        st.push_back(nxt);
                        used[nxt - 1] = true;
                        if let Some(ret) = Self::backtrack(pattern, pos + 1, nxt, st, used) {
                            return Some(ret);
                        }
                        st.pop_back();
                        used[nxt - 1] = false;
                    }
                    nxt -= 1;
                }
            }
            b'I' => {
                let mut nxt = next + 1;
                while nxt != 10 {
                    if !used[nxt - 1] {
                        st.push_back(nxt);
                        used[nxt - 1] = true;
                        if let Some(ret) = Self::backtrack(pattern, pos + 1, nxt, st, used) {
                            return Some(ret);
                        }
                        st.pop_back();
                        used[nxt - 1] = false;
                    }
                    nxt += 1;
                }
            }
            _ => panic!("impossible"),
        }
        None
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
        let actual = Solution::smallest_number("IIIDIDDD".into());
        assert_eq!(actual, "123549876");
    }

    #[test]
    fn test_case2() {
        let actual = Solution::smallest_number("DDD".into());
        assert_eq!(actual, "4321");
    }
}
