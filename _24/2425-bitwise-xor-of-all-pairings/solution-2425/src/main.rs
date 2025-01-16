impl Solution {
    #[allow(dead_code)]
    pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        nums1.iter().fold(0, |mut a, x| {
            a ^= nums2.iter().fold(0, |mut b, y| {
                b ^= x ^ y;
                b
            });
            a
        })
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
        let actual = Solution::xor_all_nums(vec![2, 1, 3], vec![10, 2, 5, 0]);
        assert_eq!(actual, 13);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::xor_all_nums(vec![1, 2], vec![3, 4]);
        assert_eq!(actual, 0);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::xor_all_nums(vec![8, 6, 29, 2, 26, 16, 15, 29], vec![24, 12, 12]);
        assert_eq!(actual, 9);
    }
}
