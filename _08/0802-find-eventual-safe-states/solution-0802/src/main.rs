impl Solution {
    #[allow(dead_code)]
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let mut r = graph.iter().enumerate().fold(vec![], |mut acc, (i, x)| {
            if x.is_empty() {
                acc.push(i as i32);
            }
            acc
        });
        for (idx, next) in graph.iter().enumerate() {
            let idx = idx as i32;
            if r.contains(&idx) {
                continue;
            }
            let mut paths: Vec<Vec<_>> = next.iter().map(|x| vec![idx, *x]).collect();
            let mut add = true;
            'outer: while !paths.is_empty() {
                for path in std::mem::take(&mut paths).iter() {
                    let last = *path.last().unwrap();
                    if r.contains(&last) {
                        continue;
                    }
                    for g in graph[last as usize].iter() {
                        if path.contains(g) {
                            add = false;
                            break 'outer;
                        }
                        let mut new_path = path.clone();
                        new_path.push(*g);
                        paths.push(new_path);
                    }
                }
            }
            if add {
                r.push(idx);
            }
        }
        r.sort();
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
        let actual = Solution::eventual_safe_nodes(vec![
            vec![1, 2],
            vec![2, 3],
            vec![5],
            vec![0],
            vec![5],
            vec![],
            vec![],
        ]);
        assert_eq!(actual, vec![2, 4, 5, 6]);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::eventual_safe_nodes(vec![
            vec![1, 2, 3, 4],
            vec![1, 2],
            vec![3, 4],
            vec![0, 4],
            vec![],
        ]);
        assert_eq!(actual, vec![4]);
    }

    #[test]
    fn test_case3() {
        let actual =
            Solution::eventual_safe_nodes(vec![vec![], vec![0, 2, 3, 4], vec![3], vec![4], vec![]]);
        assert_eq!(actual, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_case4() {
        let actual = Solution::eventual_safe_nodes(vec![
            vec![0],
            vec![2, 3, 4],
            vec![3, 4],
            vec![0, 4],
            vec![],
        ]);
        assert_eq!(actual, vec![4]);
    }
}
