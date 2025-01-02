impl Solution {
    pub fn vowel_strings(w: Vec<String>, q: Vec<Vec<i32>>) -> Vec<i32> {
        let v = ['a', 'e', 'i', 'o', 'u'];
        let mut d: Vec<i32> = vec![0; w.len()];
        w.iter().enumerate().for_each(|(i, s)| {
            let s: Vec<_> = s.chars().collect();
            let c = if !v.contains(&s[0]) || !v.contains(&s[s.len() - 1]) {
                0
            } else {
                1
            };
            d[i] = c + if i == 0 { 0 } else { d[i - 1] };
        });
        q.iter()
            .map(|i| d[i[1] as usize] - if i[0] == 0 { 0 } else { d[(i[0] - 1) as usize] })
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
