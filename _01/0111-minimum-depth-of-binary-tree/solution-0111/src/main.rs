use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn traverse(node: &Option<Rc<RefCell<TreeNode>>>, level: i32, res: &mut i32) {
        match node {
            Some(node_ref) => {
                let node_item = node_ref.borrow();
                if node_item.left.is_none()
                    && node_item.right.is_none()
                    && (*res == 0 || *res > level)
                {
                    *res = level;
                }
                Self::traverse(&node_item.left, level + 1, res);
                Self::traverse(&node_item.right, level + 1, res);
            }
            _ => {}
        }
    }

    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        Self::traverse(&root, 1, &mut res);
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

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::TreeNode;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_case1() {
        let root = Rc::new(RefCell::new(TreeNode::new(3)));
        let node10 = Rc::new(RefCell::new(TreeNode::new(9)));
        let node11 = Rc::new(RefCell::new(TreeNode::new(20)));
        let node20 = Rc::new(RefCell::new(TreeNode::new(15)));
        let node21 = Rc::new(RefCell::new(TreeNode::new(7)));

        root.borrow_mut().left = Some(Rc::clone(&node10));
        root.borrow_mut().right = Some(Rc::clone(&node11));
        node11.borrow_mut().left = Some(node20);
        node11.borrow_mut().right = Some(node21);

        let actual = Solution::min_depth(Some(root));
        assert_eq!(actual, 2);
    }
}

struct Solution;

fn main() {
    println!("Hello, world!");
}
