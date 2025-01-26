use std::collections::HashSet;

impl Solution {
    #[allow(dead_code)]
    pub fn maximum_invitations(favs: Vec<i32>) -> i32 {
        Self::case1_loop(&favs).max(Self::case2_arms(&favs))
    }

    fn case1_loop(favs: &[i32]) -> i32 {
        let (mut visited, mut res) = (vec![false; favs.len()], 0);
        for fav in 0..favs.len() as i32 {
            if visited[fav as usize] {
                continue;
            }

            let (mut curr_set, mut curr_person) = (HashSet::new(), fav);
            while !visited[curr_person as usize] {
                visited[curr_person as usize] = true;
                curr_set.insert(curr_person);
                curr_person = favs[curr_person as usize];
            }

            if curr_set.contains(&curr_person) {
                let (mut other_person, mut other_res) = (fav, curr_set.len() as i32);
                while other_person != curr_person {
                    other_res -= 1;
                    other_person = favs[other_person as usize];
                }
                res = res.max(other_res);
            }
        }
        res
    }

    fn case2_arms(favs: &[i32]) -> i32 {
        if favs.len() <= 2 {
            return favs.len() as i32;
        }

        let (mut pairs, mut checked) = (vec![], vec![false; favs.len()]);
        for fav in 0..favs.len() as i32 {
            if favs[favs[fav as usize] as usize] == fav && !checked[fav as usize] {
                pairs.push((fav, favs[fav as usize]));
                checked[fav as usize] = true;
                checked[favs[fav as usize] as usize] = true;
            }
        }

        let mut links = vec![vec![]; favs.len()];
        for (src, &dst) in favs.iter().enumerate() {
            links[dst as usize].push(src as i32);
        }

        let mut res = 0;
        for pair in pairs.iter() {
            let mut curr_maxs = vec![];

            for &(person_a, person_b) in [(pair.0, pair.1), (pair.1, pair.0)].iter() {
                let mut curr_max = 0;
                let mut queue: Vec<_> = links[person_a as usize]
                    .iter()
                    .copied()
                    .filter(|x| *x != person_b)
                    .map(|x| (1, x))
                    .collect();
                while !queue.is_empty() {
                    if let Some(curr) = queue.pop() {
                        curr_max = curr_max.max(curr.0);
                        queue.extend(
                            links[curr.1 as usize]
                                .iter()
                                .copied()
                                .map(|x| (curr.0 + 1, x)),
                        );
                    }
                }
                curr_maxs.push(curr_max);
            }

            res += 2 + curr_maxs[0] + curr_maxs[1];
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
        let actual = Solution::maximum_invitations(vec![2, 2, 1, 2]);
        assert_eq!(actual, 3);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::maximum_invitations(vec![1, 2, 0]);
        assert_eq!(actual, 3);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::maximum_invitations(vec![3, 0, 1, 4, 1]);
        assert_eq!(actual, 4);
    }

    #[test]
    fn test_case4() {
        let actual = Solution::maximum_invitations(vec![1, 0, 0, 2, 1, 4, 7, 8, 9, 6, 7, 10, 8]);
        assert_eq!(actual, 6);
    }

    #[test]
    fn test_case5() {
        let actual =
            Solution::maximum_invitations(vec![1, 0, 3, 2, 5, 6, 7, 4, 9, 8, 11, 10, 11, 12, 10]);
        assert_eq!(actual, 11);
    }
}
