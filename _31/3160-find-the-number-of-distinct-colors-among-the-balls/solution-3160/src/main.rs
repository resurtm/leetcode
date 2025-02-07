use std::collections::HashMap;

impl Solution {
    #[allow(dead_code)]
    pub fn query_results(_limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let (mut balls, mut colors) = (HashMap::new(), HashMap::new());
        let (mut total, mut res) = (0, vec![]);
        for query in queries.iter() {
            let (ball, new_color) = (query[0], query[1]);
            let curr_color = *balls.get(&ball).unwrap_or(&0);
            if curr_color != 0 {
                colors
                    .entry(curr_color)
                    .and_modify(|e| {
                        if *e == 1 {
                            total -= 1;
                        }
                        *e -= 1
                    })
                    .or_insert(0);
            }
            balls.insert(ball, new_color);
            colors
                .entry(new_color)
                .and_modify(|e| {
                    if *e == 0 {
                        total += 1;
                    }
                    *e += 1
                })
                .or_insert_with(|| {
                    total += 1;
                    1
                });
            res.push(total);
        }
        res
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
        let actual =
            Solution::query_results(4, vec![vec![1, 4], vec![2, 5], vec![1, 3], vec![3, 4]]);
        assert_eq!(actual, [1, 2, 2, 3]);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::query_results(
            4,
            vec![vec![0, 1], vec![1, 2], vec![2, 2], vec![3, 4], vec![4, 5]],
        );
        assert_eq!(actual, [1, 2, 2, 3, 4]);
    }
}
