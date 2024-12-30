impl Solution {
    fn func(i: i32, zero: i32, one: i32, cache: &mut Vec<i32>) -> i32 {
        if i == 0 {
            return 1;
        }
        if i < 0 {
            return 0;
        }
        if cache[i as usize] != -1 {
            return cache[i as usize];
        }
        let ret = Self::func(i - zero, zero, one, cache) + Self::func(i - one, zero, one, cache);
        cache[i as usize] = ret % 1_000_000_007;
        ret % 1_000_000_007
    }

    pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
        let mut cache = vec![];
        cache.resize((high + 1) as usize, -1);
        let mut ret = 0;
        for i in low..high + 1 {
            ret = (ret % 1_000_000_007) + Self::func(i, zero, one, &mut cache) % 1_000_000_007;
        }
        ret % 1_000_000_007
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
        let actual = Solution::count_good_strings(3, 3, 1, 1);
        assert_eq!(actual, 8);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::count_good_strings(2, 3, 1, 2);
        assert_eq!(actual, 5);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::count_good_strings(200, 200, 10, 1);
        assert_eq!(actual, 764_262_396);
    }
}
