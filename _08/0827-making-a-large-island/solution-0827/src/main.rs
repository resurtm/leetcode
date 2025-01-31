use std::collections::{HashMap, HashSet};

impl Solution {
    #[allow(dead_code)]
    pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
        let (w, h) = (grid.len() as i32, grid[0].len() as i32);
        let mut neighbors = vec![vec![-2; h as usize]; w as usize];

        let mut viz: HashSet<(i32, i32)> = HashSet::new();
        let mut islands = vec![];

        let mut parents: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
        let mut sizes: HashMap<(i32, i32), i32> = HashMap::new();
        for i in 0..w {
            for j in 0..h {
                if grid[i as usize][j as usize] == 0 {
                    neighbors[i as usize][j as usize] = -1;
                    continue;
                }
                parents.insert((i, j), (i, j));
                sizes.insert((i, j), 1);
                if !viz.contains(&(i, j)) {
                    let island = Self::bfs((i, j), &grid, &mut neighbors, islands.len() as i32);
                    viz.extend(island.iter());
                    islands.push(island);
                }
            }
        }

        for island in islands.iter() {
            let mut root = (-1, -1);
            for &curr in island {
                if root == (-1, -1) {
                    root = curr;
                } else {
                    Self::unify(root, curr, &mut parents, &mut sizes);
                }
            }
        }

        let mut res = *sizes.values().max().unwrap_or(&0);

        let mut checked_dirs = vec![];
        for i in 0..w {
            for j in 0..h {
                if grid[i as usize][j as usize] != 0 {
                    continue;
                }
                let dirs: HashSet<_> = [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)]
                    .iter()
                    .filter(|(m, n)| *m >= 0 && *n >= 0 && *m < w && *n < h)
                    .map(|(m, n)| neighbors[*m as usize][*n as usize])
                    .filter(|neighbor| *neighbor >= 0)
                    .collect();
                if checked_dirs.contains(&dirs) {
                    continue;
                }
                let (mut parents_cl, mut sizes_cl) = (parents.clone(), sizes.clone());
                if !dirs.is_empty() {
                    let main_dir = *islands[*dirs.iter().next().unwrap() as usize]
                        .iter()
                        .next()
                        .unwrap();
                    for &dir in dirs.iter() {
                        Self::unify(
                            main_dir,
                            *islands[dir as usize].iter().next().unwrap(),
                            &mut parents_cl,
                            &mut sizes_cl,
                        );
                    }
                }
                res = res.max(*sizes_cl.values().max().unwrap_or(&0) + 1);
                checked_dirs.push(dirs);
            }
        }

        res
    }

    fn bfs(
        start: (i32, i32),
        grid: &[Vec<i32>],
        neighbors: &mut [Vec<i32>],
        curr_island: i32,
    ) -> HashSet<(i32, i32)> {
        let (w, h) = (grid.len() as i32, grid[0].len() as i32);
        let mut queue = vec![start];
        let mut viz = HashSet::from([start]);
        neighbors[start.0 as usize][start.1 as usize] = curr_island;
        while !queue.is_empty() {
            if let Some(curr) = queue.pop() {
                let dirs: Vec<_> = [
                    (curr.0 - 1, curr.1),
                    (curr.0 + 1, curr.1),
                    (curr.0, curr.1 - 1),
                    (curr.0, curr.1 + 1),
                ]
                .iter()
                .filter(|(m, n)| *m >= 0 && *n >= 0 && *m < w && *n < h)
                .filter(|(m, n)| grid[*m as usize][*n as usize] == 1)
                .filter(|(m, n)| !viz.contains(&(*m, *n)))
                .map(|(m, n)| (*m, *n))
                .collect();
                for &dir in dirs.iter() {
                    queue.push(dir);
                    viz.insert(dir);
                    neighbors[dir.0 as usize][dir.1 as usize] = curr_island;
                }
            }
        }
        viz
    }

    fn find(to_find: (i32, i32), parents: &HashMap<(i32, i32), (i32, i32)>) -> (i32, i32) {
        let mut curr = to_find;
        while curr != parents[&curr] {
            curr = parents[&curr];
        }
        curr
    }

    fn unify(
        a: (i32, i32),
        b: (i32, i32),
        parents: &mut HashMap<(i32, i32), (i32, i32)>,
        sizes: &mut HashMap<(i32, i32), i32>,
    ) {
        if a == b {
            return;
        }
        let (root1, root2) = (Self::find(a, parents), Self::find(b, parents));
        if root1 == root2 {
            return;
        }
        if sizes[&root1] < sizes[&root2] {
            sizes.insert(root2, sizes[&root1] + sizes[&root2]);
            parents.insert(root1, root2);
        } else {
            sizes.insert(root1, sizes[&root1] + sizes[&root2]);
            parents.insert(root2, root1);
        }
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
        let actual = Solution::largest_island(vec![vec![1, 0], vec![0, 1]]);
        assert_eq!(actual, 3);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::largest_island(vec![vec![1, 1], vec![1, 0]]);
        assert_eq!(actual, 4);
    }

    #[test]
    fn test_case3() {
        let actual = Solution::largest_island(vec![vec![1, 1], vec![1, 1]]);
        assert_eq!(actual, 4);
    }

    #[test]
    fn test_case4() {
        let actual = Solution::largest_island(vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 1, 1]]);
        assert_eq!(actual, 7);
    }

    #[test]
    fn test_case5() {
        let actual = Solution::largest_island(vec![vec![0, 0], vec![0, 0]]);
        assert_eq!(actual, 1);
    }
}
