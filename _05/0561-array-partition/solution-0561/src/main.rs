impl Solution {
    #[allow(dead_code)]
    pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums.clone();
        nums.sort();
        let mut x = 0;
        for c in nums.chunks(2) {
            x += c[0].min(c[1]);
        }
        x
    }
}

#[allow(dead_code)]
struct Solution {}

fn main() {
    println!("nothing here");
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case1() {
        let actual = Solution::array_pair_sum(vec![1, 4, 3, 2]);
        assert_eq!(actual, 4);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::array_pair_sum(vec![6, 2, 6, 5, 1, 2]);
        assert_eq!(actual, 9);
    }
}
