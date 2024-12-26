use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn traverse(
        left: &Option<Rc<RefCell<TreeNode>>>,
        right: &Option<Rc<RefCell<TreeNode>>>,
        res: &mut i32,
    ) {
        if let Some(left_ref) = left {
            *res += 1;
            let left_node = left_ref.borrow();
            Self::traverse(&left_node.left, &left_node.right, res);
        }
        if let Some(right_ref) = right {
            *res += 1;
            let right_node = right_ref.borrow();
            Self::traverse(&right_node.left, &right_node.right, res);
        }
    }

    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        if let Some(root_ref) = root {
            res += 1;
            let root_node = root_ref.borrow();
            Self::traverse(&root_node.left, &root_node.right, &mut res);
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

        let actual = Solution::count_nodes(Some(root));
        assert_eq!(actual, 5);
    }
}

struct Solution;

fn main() {
    println!("Hello, world!");
}
