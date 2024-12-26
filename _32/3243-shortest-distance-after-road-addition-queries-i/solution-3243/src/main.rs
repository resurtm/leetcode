impl Solution {
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut pos = Vec::new();
        let mut dist = Vec::new();
        for i in 0..n {
            let mut it = Vec::new();
            if i < n - 1 {
                it.push(i + 1);
            }
            pos.push(it);
            dist.push(i);
        }
        let mut res: Vec<i32> = Vec::new();
        for query in queries.iter() {
            let src = query[0];
            let dst = query[1];
            pos[src as usize].push(dst);
            if dist[src as usize] + 1 < dist[dst as usize] {
                let mut q: Vec<i32> = Vec::new();
                q.push(dst);
                dist[dst as usize] = dist[src as usize] + 1;
                while !q.is_empty() {
                    let idx = q.remove(0);
                    for p in pos[idx as usize].iter() {
                        if dist[idx as usize] + 1 < dist[*p as usize] {
                            dist[*p as usize] = dist[idx as usize] + 1;
                            q.push(*p);
                        }
                    }
                }
            }
            res.push(*dist.last().unwrap());

            /*let mut q: Vec<i32> = Vec::new();
            pos[0].iter().for_each(|&x| q.push(x));
            let mut steps = 0;
            loop {
                if q.contains(&(n - 1)) {
                    break;
                }
                let mut qn: Vec<i32> = Vec::new();
                for qi in q.iter() {
                    pos[*qi as usize].iter().for_each(|&x| qn.push(x));
                }
                q = qn;
                steps += 1;
            }
            res.push(steps + 1);*/
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case1() {
        let actual =
            Solution::shortest_distance_after_queries(5, vec![vec![2, 4], vec![0, 2], vec![0, 4]]);
        assert_eq!(actual, vec![3, 2, 1]);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::shortest_distance_after_queries(4, vec![vec![0, 3], vec![0, 2]]);
        assert_eq!(actual, vec![1, 1]);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::shortest_distance_after_queries(6, vec![vec![1, 4], vec![0, 2]]);
        assert_eq!(actual, vec![3, 3]);
    }

    #[test]
    fn test_case4() {
        let actual = Solution::shortest_distance_after_queries(8, vec![vec![5, 7], vec![0, 6]]);
        assert_eq!(actual, vec![6, 2]);
    }
}

struct Solution;

fn main() {
    println!("Hello, world!");
}
