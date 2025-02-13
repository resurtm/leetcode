impl Solution {
    #[allow(dead_code)]
    pub fn capture_forts(forts: Vec<i32>) -> i32 {
        forts
            .iter()
            .fold((0, 0, 0), |a, x| {
                let (mut s, mut c, mut r) = a;
                match x {
                    -1 => {
                        if s == 1 {
                            r = r.max(c);
                        }
                        c = 0;
                        s = -1;
                    }
                    1 => {
                        if s == -1 {
                            r = r.max(c);
                        }
                        c = 0;
                        s = 1;
                    }
                    _ => {
                        c += 1;
                    }
                }
                (s, c, r)
            })
            .2
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
        let actual = Solution::capture_forts(vec![1, 0, 0, -1, 0, 0, 0, 0, 1]);
        assert_eq!(actual, 4);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::capture_forts(vec![0, 0, 1, -1]);
        assert_eq!(actual, 0);
    }
}
