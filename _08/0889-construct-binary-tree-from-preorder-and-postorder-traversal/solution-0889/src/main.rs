use std::{cell::RefCell, collections::VecDeque, rc::Rc};

impl Solution {
    #[allow(dead_code)]
    pub fn construct_from_pre_post(
        preorder: Vec<i32>,
        postorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut res = None;
        let mut st: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        let mut pos = 0;

        for &it0 in preorder.iter() {
            let node = Rc::new(RefCell::new(TreeNode::new(it0)));
            let mut to_add: Option<Rc<RefCell<TreeNode>>> = None;

            if let Some(last) = st.back() {
                let mut last_node = last.borrow_mut();
                if last_node.left.is_none() {
                    last_node.left = Some(Rc::clone(&node));
                } else {
                    last_node.right = Some(Rc::clone(&node));
                }
                to_add = Some(node);
            } else {
                res = Some(Rc::clone(&node));
                st.push_back(node);
                continue;
            }

            if let Some(t) = to_add {
                st.push_back(t);
            }

            while !st.is_empty() && pos < postorder.len() {
                let last = if let Some(last) = st.back() {
                    last.borrow().val
                } else {
                    -1
                };
                if last != postorder[pos] {
                    break;
                }
                st.pop_back();
                pos += 1;
            }
        }

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
        let node1 = Rc::new(RefCell::new(TreeNode::new(1)));
        let node2 = Rc::new(RefCell::new(TreeNode::new(2)));
        let node3 = Rc::new(RefCell::new(TreeNode::new(3)));
        let node4 = Rc::new(RefCell::new(TreeNode::new(4)));
        let node5 = Rc::new(RefCell::new(TreeNode::new(5)));
        let node6 = Rc::new(RefCell::new(TreeNode::new(6)));
        let node7 = Rc::new(RefCell::new(TreeNode::new(7)));

        node1.borrow_mut().left = Some(Rc::clone(&node2));
        node1.borrow_mut().right = Some(Rc::clone(&node3));
        node2.borrow_mut().left = Some(Rc::clone(&node4));
        node2.borrow_mut().right = Some(Rc::clone(&node5));
        node3.borrow_mut().left = Some(Rc::clone(&node6));
        node3.borrow_mut().right = Some(Rc::clone(&node7));

        assert_eq!(
            Solution::construct_from_pre_post(vec![1, 2, 4, 5, 3, 6, 7], vec![4, 5, 2, 6, 7, 3, 1]),
            Some(node1)
        );
    }
}
