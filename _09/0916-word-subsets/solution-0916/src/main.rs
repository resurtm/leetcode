use std::collections::HashMap;

impl Solution {
    pub fn word_subsets(ws0: Vec<String>, ws1: Vec<String>) -> Vec<String> {
        let ws1: Vec<HashMap<_, _>> = ws1
            .iter()
            .map(|w1| {
                w1.bytes().fold(HashMap::new(), |mut mt, it| {
                    mt.entry(it).and_modify(|n| *n += 1).or_insert(1);
                    mt
                })
            })
            .collect();
        ws0.iter().fold(Vec::new(), |mut r, w0| {
            let ma = w0.bytes().fold(HashMap::new(), |mut mt, it| {
                mt.entry(it).and_modify(|n| *n += 1).or_insert(1);
                mt
            });
            for w1 in ws1.iter() {
                for (ch, nb) in w1.iter() {
                    if *ma.get(ch).unwrap_or(&0) < *nb {
                        return r;
                    }
                }
            }
            r.push(w0.to_owned());
            r
        })
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
        let actual = Solution::word_subsets(
            vec![
                "amazon".into(),
                "apple".into(),
                "facebook".into(),
                "google".into(),
                "leetcode".into(),
            ],
            vec!["e".into(), "o".into()],
        );
        assert_eq!(
            actual,
            vec![
                String::from("facebook"),
                String::from("google"),
                String::from("leetcode")
            ]
        );
    }

    #[test]
    fn test_case2() {
        let actual = Solution::word_subsets(
            vec![
                "amazon".into(),
                "apple".into(),
                "facebook".into(),
                "google".into(),
                "leetcode".into(),
            ],
            vec!["l".into(), "e".into()],
        );
        assert_eq!(
            actual,
            vec![
                String::from("apple"),
                String::from("google"),
                String::from("leetcode")
            ]
        );
    }
}
