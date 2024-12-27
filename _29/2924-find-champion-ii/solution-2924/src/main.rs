use std::collections::HashMap;

impl Solution {
    pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut teams = HashMap::new();
        for x in 0..n {
            teams.insert(x, 0);
        }
        for edge in edges.iter() {
            let a = edge[0];
            let b = edge[1];

            // let ap = teams.get(&a);
            // teams.insert(a, ap.unwrap() - 1);

            let bp = teams.get(&b);
            teams.insert(b, bp.unwrap() + 1);
        }
        let min = teams.iter().min_by(|x, y| x.1.cmp(y.1)).unwrap();
        let minc = teams.iter().filter(|&x| x.1 == min.1).count();
        println!("{:?}", teams);
        if minc != 1 {
            -1
        } else {
            *min.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case1() {
        let actual = Solution::find_champion(3, vec![vec![0, 1], vec![1, 2]]);
        assert_eq!(actual, 0);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::find_champion(4, vec![vec![0, 2], vec![1, 3], vec![1, 2]]);
        assert_eq!(actual, -1);
    }
}

struct Solution;

fn main() {
    println!("Hello, world!");
}
