impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        match n {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => 5,
            n => {
                let mut a = 3;
                let mut b = 5;
                let mut c = a + b;
                for _ in 0..n - 5 {
                    a = b;
                    b = c;
                    c = a + b;
                }
                c
            }
        }
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
        let actual = Solution::climb_stairs(2);
        assert_eq!(actual, 2);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::climb_stairs(3);
        assert_eq!(actual, 3);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::climb_stairs(6);
        assert_eq!(actual, 13);
    }
}
