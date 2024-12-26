impl Solution {
    fn traverse(ops_inp: &Vec<char>, lev: i32, nums: &Vec<i32>, target: i32, res: &mut i32) {
        if lev == nums.len() as i32 {
            let found = nums.iter().enumerate().fold(0, |acc, (idx, &it)| {
                acc + if ops_inp[idx] == '+' { it } else { -it }
            });
            if found == target {
                *res += 1;
            }
            return;
        }
        ['-', '+'].iter().for_each(|&op| {
            let mut ops_next = ops_inp.clone();
            ops_next.push(op);
            Self::traverse(&ops_next, lev + 1, nums, target, res);
        });
    }

    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let mut res = 0;
        ['-', '+']
            .iter()
            .for_each(|&op| Self::traverse(&vec![op], 1, &nums, target, &mut res));
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case1() {
        let actual = Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3);
        assert_eq!(actual, 5);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::find_target_sum_ways(vec![1], 1);
        assert_eq!(actual, 1);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::find_target_sum_ways(
            vec![
                8, 48, 11, 47, 26, 12, 16, 39, 38, 50, 21, 12, 34, 1, 28, 1, 3, 9, 17, 50,
            ],
            3,
        );
        assert_eq!(actual, 6317);
    }
}

struct Solution;

fn main() {
    let res = Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3);
    println!("res1: {}", res);
    let res = Solution::find_target_sum_ways(vec![1], 1);
    println!("res2: {}", res);
    let res = Solution::find_target_sum_ways(
        vec![
            8, 48, 11, 47, 26, 12, 16, 39, 38, 50, 21, 12, 34, 1, 28, 1, 3, 9, 17, 50,
        ],
        3,
    );
    println!("res3: {}", res);
}
