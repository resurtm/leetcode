impl Solution {
    #[allow(dead_code)]
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (mut i, mut j, mut res) = (0, 0, vec![]);
        while i < nums1.len() || j < nums2.len() {
            if i < nums1.len() && j < nums2.len() {
                match nums1[i][0].cmp(&nums2[j][0]) {
                    std::cmp::Ordering::Less => {
                        res.push(nums1[i].clone());
                        i += 1;
                    }
                    std::cmp::Ordering::Greater => {
                        res.push(nums2[j].clone());
                        j += 1;
                    }
                    std::cmp::Ordering::Equal => {
                        res.push(vec![nums1[i][0], nums1[i][1] + nums2[j][1]]);
                        i += 1;
                        j += 1;
                    }
                }
            } else if i < nums1.len() {
                res.push(nums1[i].clone());
                i += 1;
            } else if j < nums2.len() {
                res.push(nums2[j].clone());
                j += 1;
            }
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
        let actual = Solution::merge_arrays(
            vec![vec![1, 2], vec![2, 3], vec![4, 5]],
            vec![vec![1, 4], vec![3, 2], vec![4, 1]],
        );
        assert_eq!(actual, [[1, 6], [2, 3], [3, 2], [4, 6]]);
    }

    #[test]
    fn test_case2() {
        let actual = Solution::merge_arrays(
            vec![vec![2, 4], vec![3, 6], vec![5, 5]],
            vec![vec![1, 3], vec![4, 3]],
        );
        assert_eq!(actual, [[1, 3], [2, 4], [3, 6], [4, 3], [5, 5]]);
    }
}
