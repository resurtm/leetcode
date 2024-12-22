use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::i32;

impl Solution {
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<char>,
        changed: Vec<char>,
        cost: Vec<i32>,
    ) -> i64 {
        // let mut m0: HashMap<(char, char), i32> = HashMap::new();
        // let mut m1: HashMap<char, Vec<(char, i32)>> = HashMap::new();
        // for (idx, &it) in cost.iter().enumerate() {
        //     m0.insert((original[idx], changed[idx]), it);
        //     if let Some(ex) = m1.get(&original[idx]) {
        //         let mut new = ex.clone();
        //         new.push((changed[idx], it));
        //         m1.insert(original[idx], new);
        //     } else {
        //         m1.insert(original[idx], vec![(changed[idx], it)]);
        //     }
        // }
        let mut cache: HashMap<(char, char), i32> = HashMap::new();

        // println!("{}", Self::bfs('b', 'c', &original, &changed, &cost));
        // return 0;

        // Self::dijkstra('c', 'b', &original, &changed, &cost);
        // return 0;

        let mut res = 0i64;
        for (idx, ch0) in source.chars().enumerate() {
            let ch1 = target.chars().nth(idx).unwrap();
            if ch0 == ch1 {
                continue;
            }
            // let cost = Self::bfs2(ch0, ch1, &m1, &mut cache);
            // let cost = Self::bfs(ch0, ch1, &original, &changed, &cost);
            let cost = Self::dijkstra(ch0, ch1, &original, &changed, &cost, &mut cache);
            if cost == 0 {
                return -1;
            }
            res += cost as i64
        }
        res
    }

    fn dijkstra(
        src: char,
        dst: char,
        original: &[char],
        changed: &[char],
        cost: &[i32],
        cache: &mut HashMap<(char, char), i32>,
    ) -> i32 {
        if let Some(&cached) = cache.get(&(src, dst)) {
            return cached;
        }
        let mut viz = HashSet::new();
        let mut dist: HashMap<char, i32> = HashMap::new();
        dist.insert(src, 0);
        let mut pq = BinaryHeap::new();
        pq.push(Reverse((0, src)));
        while !pq.is_empty() {
            let curr = pq.pop().unwrap().0;
            viz.insert(curr.1);
            for (edge_idx, edge_ch0) in original.iter().enumerate() {
                if curr.1 != *edge_ch0 {
                    continue;
                }
                let edge_ch = changed[edge_idx];
                if viz.contains(&edge_ch) {
                    continue;
                }
                let new_dist = *dist.get(&curr.1).unwrap_or(&i32::MAX) + cost[edge_idx];
                let prev_dist = *dist.get(&edge_ch).unwrap_or(&i32::MAX);
                if new_dist < prev_dist {
                    dist.insert(edge_ch, new_dist);
                    pq.push(Reverse((new_dist, edge_ch)));
                }
            }
        }
        // println!("{:?}", dist);
        // println!("{}", dist[&dst]);
        let res = *dist.get(&dst).unwrap_or(&0);
        cache.insert((src, dst), res);
        res
    }

    fn bfs(src: char, dst: char, original: &[char], changed: &[char], cost: &[i32]) -> i32 {
        let mut routes = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_front(vec![(src, 0)]);
        while !queue.is_empty() {
            if let Some(curr) = queue.pop_front() {
                let last = curr.last().unwrap();
                for (original_idx, &original_it) in original.iter().enumerate() {
                    if original_it != last.0 {
                        continue;
                    }
                    let changed_it = changed[original_idx];
                    let cost_it = cost[original_idx];
                    if curr.iter().position(|x| x.0 == changed_it).is_none() {
                        if changed_it == dst {
                            routes.push(last.1 + cost_it);
                        } else {
                            let mut next = curr.clone();
                            next.push((changed_it, last.1 + cost_it));
                            queue.push_back(next);
                        }
                    }
                }
            }
        }
        // println!("{:#?}", routes);
        *routes.iter().min().unwrap_or(&0)
    }

    fn bfs2(
        a: char,
        b: char,
        m: &HashMap<char, Vec<(char, i32)>>,
        cache: &mut HashMap<(char, char), i32>,
    ) -> i32 {
        if let Some(&cached) = cache.get(&(a, b)) {
            return cached;
        }
        let mut routes = vec![];
        let mut q = VecDeque::new();
        q.push_front(vec![(a, 0)]);
        while !q.is_empty() {
            if let Some(curr) = q.pop_front() {
                let last = curr.last().unwrap();
                if let Some(ex) = m.get(&last.0) {
                    for it in ex.iter() {
                        if curr.contains(it) {
                            continue;
                        }
                        let mut next = curr.clone();
                        next.push((it.0, it.1));
                        if it.0 == b {
                            routes.push(next);
                        } else {
                            q.push_back(next);
                        }
                    }
                }
            }
        }
        let res = routes
            .iter()
            .map(|x| x.iter().fold(0, |acc, x| acc + x.1))
            .min()
            .unwrap_or(0);
        cache.insert((a, b), res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case1() {
        let source = String::from("abcd");
        let target = String::from("acbe");
        let original = vec!['a', 'b', 'c', 'c', 'e', 'd'];
        let changed = vec!['b', 'c', 'b', 'e', 'b', 'e'];
        let cost = vec![2, 5, 5, 1, 2, 20];
        let actual = Solution::minimum_cost(source, target, original, changed, cost);
        assert_eq!(actual, 28);
    }

    #[test]
    fn test_case2() {
        let source = String::from("aaaa");
        let target = String::from("bbbb");
        let original = vec!['a', 'c'];
        let changed = vec!['c', 'b'];
        let cost = vec![1, 2];
        let actual = Solution::minimum_cost(source, target, original, changed, cost);
        assert_eq!(actual, 12);
    }

    #[test]
    fn test_case3() {
        let source = String::from("abcd");
        let target = String::from("acbe");
        let original = vec!['a'];
        let changed = vec!['e'];
        let cost = vec![10_000];
        let actual = Solution::minimum_cost(source, target, original, changed, cost);
        assert_eq!(actual, -1);
    }
}

struct Solution;

fn main() {
    let source = String::from("abcd");
    let target = String::from("acbe");
    let original = vec!['a', 'b', 'c', 'c', 'e', 'd'];
    let changed = vec!['b', 'c', 'b', 'e', 'b', 'e'];
    let cost = vec![2, 5, 5, 1, 2, 20];
    let actual = Solution::minimum_cost(source, target, original, changed, cost);
    println!("Result: {}", actual);
}
