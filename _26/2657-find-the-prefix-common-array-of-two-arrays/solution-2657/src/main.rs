impl Solution {
    #[allow(dead_code)]
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        (0..a.len())
            .fold((0u64, 0u64, vec![]), |(mut ma, mut mb, mut r), x| {
                ma |= 1 << a[x];
                mb |= 1 << b[x];
                r.push((ma & mb).count_ones() as i32);
                (ma, mb, r)
            })
            .2
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
        let actual = Solution::find_the_prefix_common_array(vec![1, 3, 2, 4], vec![3, 1, 2, 4]);
        assert_eq!(actual, vec![0, 2, 3, 4]);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::find_the_prefix_common_array(vec![2, 3, 1], vec![3, 1, 2]);
        assert_eq!(actual, vec![0, 1, 3]);
    }
}
