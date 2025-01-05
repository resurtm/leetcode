use std::collections::HashSet;

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let l: HashSet<u8> = HashSet::from_iter(letters.iter().map(|&x| x as u8));
        for c in target as u8 + 1..=b'z' {
            if l.contains(&c) {
                return c as char;
            }
        }
        letters[0]
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
        let actual = Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'a');
        assert_eq!(actual, 'c');
    }

    #[test]
    fn test_case2() {
        let actual = Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'c');
        assert_eq!(actual, 'f');
    }

    #[test]
    fn test_case3() {
        let actual = Solution::next_greatest_letter(vec!['x', 'x', 'y', 'y'], 'z');
        assert_eq!(actual, 'x');
    }
}
