impl Solution {
    pub fn ways_to_split_array(its: Vec<i32>) -> i32 {
        let s = its.len();
        let mut f = vec![0i128; s];
        let mut b = vec![0i128; s];
        for i in 0..s {
            f[i] = if i == 0 { 0 } else { f[i - 1] } + its[i] as i128;
            b[s - 1 - i] = if i == 0 { 0 } else { b[s - i] } + its[s - 1 - i] as i128;
        }
        let mut r = 0;
        for i in 0..s - 1 {
            if f[i] >= b[i + 1] {
                r += 1;
            }
        }
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
        let actual = Solution::ways_to_split_array(vec![10, 4, -8, 7]);
        assert_eq!(actual, 2);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::ways_to_split_array(vec![2, 3, 1, 0]);
        assert_eq!(actual, 2);
    }
}
