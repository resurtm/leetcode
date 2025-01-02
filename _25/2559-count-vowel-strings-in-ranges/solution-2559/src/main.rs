use std::collections::HashMap;

impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let vowels = vec!['a', 'e', 'i', 'o', 'u'];
        let mut freq = HashMap::new();
        words
            .iter()
            .enumerate()
            .filter(|x| {
                let c: Vec<_> = x.1.chars().collect();
                vowels.contains(&c[0]) && vowels.contains(&c[c.len() - 1])
            })
            .for_each(|x| {
                freq.insert(
                    x.0 as i32,
                    x.1.chars().fold(
                        0,
                        |acc, ch| if vowels.contains(&ch) { acc + 1 } else { acc },
                    ),
                );
            });
        queries
            .iter()
            .map(|x| {
                let mut res = 0;
                for i in x[0]..x[1] + 1 {
                    res += if freq.contains_key(&i) { 1 } else { 0 };
                }
                res
            })
            .collect()
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
        let actual = Solution::vowel_strings(
            vec![
                String::from("aba"),
                String::from("bcb"),
                String::from("ece"),
                String::from("aa"),
                String::from("e"),
            ],
            vec![vec![0, 2], vec![1, 4], vec![1, 1]],
        );
        assert_eq!(actual, vec![2, 3, 0]);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::vowel_strings(
            vec![String::from("a"), String::from("e"), String::from("i")],
            vec![vec![0, 2], vec![0, 1], vec![2, 2]],
        );
        assert_eq!(actual, vec![3, 2, 1]);
    }
}
