impl Solution {
    pub fn count_prefix_suffix_pairs(ws: Vec<String>) -> i32 {
        let mut r = 0;
        (0..ws.len()).for_each(|i| {
            (i + 1..ws.len()).for_each(|j| {
                if ws[j].starts_with(&ws[i]) && ws[j].ends_with(&ws[i]) {
                    r += 1;
                }
            });
        });
        r
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
        let actual = Solution::count_prefix_suffix_pairs(vec![
            "a".into(),
            "aba".into(),
            "ababa".into(),
            "a".into(),
        ]);
        assert_eq!(actual, 4);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::count_prefix_suffix_pairs(vec![
            "pa".into(),
            "papa".into(),
            "ma".into(),
            "mama".into(),
        ]);
        assert_eq!(actual, 2);
    }
}
