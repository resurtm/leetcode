impl Solution {
    #[allow(dead_code)]
    pub fn most_profitable_path(edges: Vec<Vec<i32>>, bob: i32, amount: Vec<i32>) -> i32 {
        let mut links = vec![vec![]; amount.len()];
        for edge in edges {
            links[edge[0] as usize].push(edge[1]);
            links[edge[1] as usize].push(edge[0]);
        }

        let mut bob_path = vec![bob];
        Self::bob_dfs(bob, -1, &mut bob_path, &links);

        let mut res = i32::MIN;
        Self::alice_dfs(0, -1, 1, amount[0], &bob_path, &links, &amount, &mut res);
        res
    }

    fn bob_dfs(curr: i32, parent: i32, path: &mut Vec<i32>, links: &[Vec<i32>]) -> bool {
        if curr == 0 {
            return true;
        }
        for &link in links[curr as usize].iter() {
            if link == parent {
                continue;
            }
            path.push(link);
            if Self::bob_dfs(link, curr, path, links) {
                return true;
            }
            path.pop();
        }
        false
    }

    fn alice_dfs(
        curr: i32,
        parent: i32,
        sec: usize,
        acc: i32,
        bob_path: &[i32],
        links: &[Vec<i32>],
        amount: &[i32],
        res: &mut i32,
    ) {
        let mut is_end = true;
        for &link in links[curr as usize].iter() {
            if link == parent {
                continue;
            }
            is_end = false;
            let mut to_add = amount[link as usize];
            let bob_sec = if sec >= bob_path.len() {
                bob_path.len() - 1
            } else {
                sec
            };
            if link == bob_path[bob_sec] {
                to_add /= 2;
            } else if bob_path[..bob_sec].contains(&link) {
                to_add = 0;
            }
            Self::alice_dfs(
                link,
                curr,
                sec + 1,
                acc + to_add,
                bob_path,
                links,
                amount,
                res,
            );
        }
        if is_end {
            *res = acc.max(*res);
        }
    }
}

struct Solution {}

fn main() {
    println!("nothing here");
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case1() {
        let actual = Solution::most_profitable_path(
            vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![3, 4]],
            3,
            vec![-2, 4, 2, -4, 6],
        );
        assert_eq!(actual, 6);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::most_profitable_path(
            vec![
                vec![0, 2],
                vec![0, 6],
                vec![1, 3],
                vec![1, 5],
                vec![2, 5],
                vec![4, 6],
            ],
            6,
            vec![8838, -6396, -5940, 2694, -1366, 4616, 2966],
        );
        assert_eq!(actual, 7472);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::most_profitable_path(vec![vec![0, 1]], 1, vec![-7280, 2350]);
        assert_eq!(actual, -7280);
    }
}
