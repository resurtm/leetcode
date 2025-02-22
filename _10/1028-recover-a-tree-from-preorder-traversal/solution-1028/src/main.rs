use std::{cell::RefCell, collections::VecDeque, rc::Rc};

impl Solution {
    #[allow(dead_code)]
    pub fn recover_from_preorder(trav: String) -> Option<Rc<RefCell<TreeNode>>> {
        let trav = Self::parse(trav);
        let mut st: VecDeque<(usize, Rc<RefCell<TreeNode>>)> = VecDeque::new();
        for it in trav {
            if let Some(ls) = st.pop_back() {
                let mut last = ls;
                if last.0 != it.0 - 1 {
                    for _ in 0..last.0 - it.0 + 1 {
                        last = st.pop_back().unwrap();
                    }
                }
                let node = Rc::new(RefCell::new(TreeNode::new(it.1)));
                let mut tmp = last.1.borrow_mut();
                if tmp.left.is_none() {
                    tmp.left = Some(Rc::clone(&node));
                } else {
                    tmp.right = Some(Rc::clone(&node));
                }
                st.push_back((last.0, Rc::clone(&last.1)));
                st.push_back((it.0, node));
            } else {
                let node = Rc::new(RefCell::new(TreeNode::new(it.1)));
                st.push_back((it.0, node));
            }
        }
        Some(Rc::clone(&st.front().unwrap().1))
    }

    fn parse(trav: String) -> Vec<(usize, i32)> {
        let (mut res, mut st, mut lev, mut acc) = (vec![], true, 0, vec![]);
        for &it in trav.as_bytes() {
            if !st && it != b'-' {
                st = true;
                lev = acc.len();
                acc.clear();
            } else if st && it == b'-' {
                res.push((
                    lev,
                    String::from_utf8_lossy(&acc).into_owned().parse().unwrap(),
                ));
                st = false;
                lev = 0;
                acc.clear();
            }
            acc.push(it);
        }
        res.push((
            lev,
            String::from_utf8_lossy(&acc).into_owned().parse().unwrap(),
        ));
        res
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution {}

fn main() {
    println!("nothing here");
}

#[cfg(test)]
mod tests {
    use super::{Solution, TreeNode};
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn test_case1() {
        let root = Rc::new(RefCell::new(TreeNode::new(1)));
        let node2 = Rc::new(RefCell::new(TreeNode::new(2)));
        let node5 = Rc::new(RefCell::new(TreeNode::new(5)));
        let node3 = Rc::new(RefCell::new(TreeNode::new(3)));
        let node6 = Rc::new(RefCell::new(TreeNode::new(6)));
        let node4 = Rc::new(RefCell::new(TreeNode::new(4)));
        let node7 = Rc::new(RefCell::new(TreeNode::new(7)));

        root.borrow_mut().left = Some(Rc::clone(&node2));
        root.borrow_mut().right = Some(Rc::clone(&node5));
        node2.borrow_mut().left = Some(Rc::clone(&node3));
        node3.borrow_mut().left = Some(Rc::clone(&node4));
        node5.borrow_mut().left = Some(Rc::clone(&node6));
        node6.borrow_mut().left = Some(Rc::clone(&node7));

        assert_eq!(
            Solution::recover_from_preorder("1-2--3---4-5--6---7".into()),
            Some(root)
        );
    }
}
