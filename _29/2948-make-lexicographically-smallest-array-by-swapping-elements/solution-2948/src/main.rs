impl Solution {
    #[allow(dead_code)]
    pub fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32> {
        let mut nums: Vec<_> = nums
            .iter()
            .enumerate()
            .map(|(pos, num)| (*num, pos))
            .collect();
        nums.sort();

        let mut acc: Vec<(i32, usize)> = vec![];
        let mut parts: Vec<Vec<(i32, usize)>> = vec![];
        for &(num, pos) in nums.iter() {
            if let Some(&last) = acc.last() {
                if num - last.0 > limit {
                    parts.push(std::mem::take(&mut acc));
                }
            }
            acc.push((num, pos));
        }
        if !acc.is_empty() {
            parts.push(std::mem::take(&mut acc));
        }

        let mut ps = vec![];
        for part in parts.iter() {
            let mut psi = vec![];
            for &(_, pos) in part.iter() {
                psi.push(pos);
            }
            psi.sort();
            ps.push(psi);
        }

        let mut r = vec![0i32; nums.len()];
        for (idx_part, part) in parts.iter().enumerate() {
            for (idx_num, &(num, _)) in part.iter().enumerate() {
                r[ps[idx_part][idx_num]] = num;
            }
        }
        r
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
        let actual = Solution::lexicographically_smallest_array(vec![1, 5, 3, 9, 8], 2);
        assert_eq!(actual, [1, 3, 5, 8, 9]);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::lexicographically_smallest_array(vec![1, 7, 6, 18, 2, 1], 3);
        assert_eq!(actual, [1, 6, 7, 18, 1, 2]);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::lexicographically_smallest_array(vec![1, 7, 28, 19, 10], 3);
        assert_eq!(actual, [1, 7, 28, 19, 10]);
    }
}
