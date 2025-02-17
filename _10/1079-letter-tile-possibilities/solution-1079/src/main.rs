use std::collections::{HashMap, HashSet};

impl Solution {
    #[allow(dead_code)]
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let mut fr = tiles.bytes().fold(HashMap::new(), |mut acc, x| {
            acc.entry(x)
                .and_modify(|e| {
                    *e += 1;
                })
                .or_insert(1);
            acc
        });
        let keys: HashSet<_> = fr.keys().cloned().collect();
        let mut res = 0;
        Self::backtrack(&mut fr, &keys, &mut res);
        res
    }

    fn backtrack(fr: &mut HashMap<u8, i32>, keys: &HashSet<u8>, res: &mut i32) {
        for x in keys.iter() {
            if fr[x] == 0 {
                continue;
            }
            fr.entry(*x).and_modify(|e| {
                *e -= 1;
            });
            *res += 1;
            Self::backtrack(fr, keys, res);
            fr.entry(*x).and_modify(|e| {
                *e += 1;
            });
        }
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
        let actual = Solution::num_tile_possibilities("AAB".into());
        assert_eq!(actual, 8);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::num_tile_possibilities("AAABBC".into());
        assert_eq!(actual, 188);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::num_tile_possibilities("V".into());
        assert_eq!(actual, 1);
    }
}
