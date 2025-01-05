impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
        let mut diff = vec![0; s.len()];
        for shift in shifts.iter() {
            if let [start, end, dir] = shift[0..3] {
                (start as usize..=end as usize).for_each(|i| {
                    diff[i] = if dir == 0 { diff[i] - 1 } else { diff[i] + 1 } % 26;
                });
            }
        }
        let mut its = s.as_bytes().to_vec();
        for (idx, it) in its.iter_mut().enumerate() {
            let itn = ((*it - b'a') as i32 + diff[idx] % 26) % 26;
            *it = (if itn < 0 { itn + 26 } else { itn }) as u8 + b'a';
        }
        String::from_utf8(its).unwrap()
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
