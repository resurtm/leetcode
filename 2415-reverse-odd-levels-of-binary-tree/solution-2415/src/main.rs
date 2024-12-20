use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn dfs(
        node0: &Option<Rc<RefCell<TreeNode>>>,
        node1: &Option<Rc<RefCell<TreeNode>>>,
        depth: i32,
    ) {
        match (node0, node1) {
            (Some(node0_ref), Some(node1_ref)) => {
                let mut node0_node = node0_ref.borrow_mut();
                let mut node1_node = node1_ref.borrow_mut();

                if depth % 2 != 0 {
                    let temp = node1_node.val;
                    node1_node.val = node0_node.val;
                    node0_node.val = temp;
                }

                Self::dfs(&node0_node.left, &node1_node.right, depth + 1);
                Self::dfs(&node0_node.right, &node1_node.left, depth + 1);
            }
            _ => {}
        }
    }

    pub fn reverse_odd_levels(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root_ref) = &root {
            let root_node = root_ref.borrow();
            Self::dfs(&root_node.left, &root_node.right, 1);
        }
        root
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
        // actual
        let root = Rc::new(RefCell::new(TreeNode::new(2)));
        let node10 = Rc::new(RefCell::new(TreeNode::new(3)));
        let node11 = Rc::new(RefCell::new(TreeNode::new(5)));
        let node20 = Rc::new(RefCell::new(TreeNode::new(8)));
        let node21 = Rc::new(RefCell::new(TreeNode::new(13)));
        let node22 = Rc::new(RefCell::new(TreeNode::new(21)));
        let node23 = Rc::new(RefCell::new(TreeNode::new(34)));

        root.borrow_mut().left = Some(Rc::clone(&node10));
        root.borrow_mut().right = Some(Rc::clone(&node11));
        node10.borrow_mut().left = Some(node20);
        node10.borrow_mut().right = Some(node21);
        node11.borrow_mut().left = Some(node22);
        node11.borrow_mut().right = Some(node23);

        let actual = Solution::reverse_odd_levels(Some(root));

        // expected
        let root = Rc::new(RefCell::new(TreeNode::new(2)));
        let node10 = Rc::new(RefCell::new(TreeNode::new(5)));
        let node11 = Rc::new(RefCell::new(TreeNode::new(3)));
        let node20 = Rc::new(RefCell::new(TreeNode::new(8)));
        let node21 = Rc::new(RefCell::new(TreeNode::new(13)));
        let node22 = Rc::new(RefCell::new(TreeNode::new(21)));
        let node23 = Rc::new(RefCell::new(TreeNode::new(34)));

        root.borrow_mut().left = Some(Rc::clone(&node10));
        root.borrow_mut().right = Some(Rc::clone(&node11));
        node10.borrow_mut().left = Some(node20);
        node10.borrow_mut().right = Some(node21);
        node11.borrow_mut().left = Some(node22);
        node11.borrow_mut().right = Some(node23);

        let expected = Some(root);

        // compare
        assert_eq!(actual, expected);
    }
}

struct Solution;

fn main() {
    println!("Hello, world!");
}
