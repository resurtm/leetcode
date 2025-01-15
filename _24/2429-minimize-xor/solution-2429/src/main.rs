impl Solution {
    #[allow(dead_code)]
    pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
        let t = num2.count_ones();
        let mut r = num1;
        let mut p = 0;
        while r.count_ones() < t {
            if r & (1 << p) == 0 {
                r |= 1 << p;
            }
            p += 1;
        }
        p = 0;
        while r.count_ones() > t {
            if r & (1 << p) != 0 {
                r &= !(1 << p);
            }
            p += 1;
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
        let actual = Solution::minimize_xor(3, 5);
        assert_eq!(actual, 3);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::minimize_xor(1, 12);
        assert_eq!(actual, 3);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::minimize_xor(31, 31);
        assert_eq!(actual, 31);
    }

    #[test]
    fn test_case4() {
        let actual = Solution::minimize_xor(79, 74);
        assert_eq!(actual, 76);
    }
}
