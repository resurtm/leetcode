impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];
        for (i, price) in prices.iter().enumerate() {
            let mut item = *price;
            for j in i + 1..prices.len() {
                if prices[j] <= prices[i] {
                    item -= prices[j];
                    break;
                }
            }
            res.push(item);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case1() {
        let actual = Solution::final_prices(vec![8, 4, 6, 2, 3]);
        assert_eq!(actual, vec![4, 2, 4, 2, 3]);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::final_prices(vec![1, 2, 3, 4, 5]);
        assert_eq!(actual, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::final_prices(vec![10, 1, 1, 6]);
        assert_eq!(actual, vec![9, 0, 1, 6]);
    }
}

struct Solution;

fn main() {
    println!("Hello, world!");
}
