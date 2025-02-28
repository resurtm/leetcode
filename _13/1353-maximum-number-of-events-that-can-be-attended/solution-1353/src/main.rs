use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
};

impl Solution {
    #[allow(dead_code)]
    pub fn max_events(events: Vec<Vec<i32>>) -> i32 {
        let (mut events, l, mut days) = (events.clone(), events.len(), events[0][1]);
        events.sort_by(|a, b| {
            days = days.max(a[1].max(b[1]));
            let x = a[0].cmp(&b[0]);
            if x == Ordering::Equal {
                a[1].cmp(&b[1])
            } else {
                x
            }
        });
        let mut h = BinaryHeap::new();
        let (mut day, mut event, mut res) = (1, 0, 0);
        while day <= days {
            // fast forward time, if no events to visit available
            if event < l && h.is_empty() {
                day = events[event][0];
            }
            // add all events starting today to heap
            while event < l && events[event][0] <= day {
                h.push(Reverse(events[event][1]));
                event += 1;
            }
            // remove all ended events from heap
            while let Some(x) = h.pop() {
                if x.0 >= day {
                    h.push(Reverse(x.0));
                    break;
                }
            }
            // visit earliest (in heap) event
            if h.pop().is_some() {
                res += 1;
            } else if event >= l {
                break;
            }
            day += 1;
        }
        res
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
        let actual = Solution::max_events(vec![vec![1, 2], vec![2, 3], vec![3, 4]]);
        assert_eq!(actual, 3);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::max_events(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 2]]);
        assert_eq!(actual, 4);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::max_events(vec![vec![1, 100000]]);
        assert_eq!(actual, 1);
    }

    #[test]
    fn test_case4() {
        let actual = Solution::max_events(vec![
            vec![1, 5],
            vec![1, 5],
            vec![1, 5],
            vec![2, 3],
            vec![2, 3],
        ]);
        assert_eq!(actual, 5);
    }

    #[test]
    fn test_case5() {
        let actual = Solution::max_events(vec![
            vec![3, 3],
            vec![1, 3],
            vec![2, 3],
            vec![3, 4],
            vec![3, 4],
        ]);
        assert_eq!(actual, 4);
    }
}
