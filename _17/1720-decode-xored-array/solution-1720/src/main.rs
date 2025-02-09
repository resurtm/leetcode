impl Solution {
    #[allow(dead_code)]
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let mut res = vec![first];
        for i in 0..encoded.len() {
            res.push(res[i] ^ encoded[i]);
        }
        res
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
        let actual = Solution::decode(vec![1, 2, 3], 1);
        assert_eq!(actual, [1, 0, 2, 1]);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::decode(vec![6, 2, 7, 3], 4);
        assert_eq!(actual, [4, 2, 0, 7, 4]);
    }
}
