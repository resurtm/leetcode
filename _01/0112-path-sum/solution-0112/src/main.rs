use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn has_path_sum(rooti: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        let Some(root) = rooti else {
            return false;
        };
        if root.borrow().left.is_none() && root.borrow().right.is_none() {
            return root.borrow().val == target_sum;
        }
        let val = root.borrow().val;
        Self::traverse(&root, val, target_sum)
    }

    fn traverse(node: &Rc<RefCell<TreeNode>>, sum: i32, target_sum: i32) -> bool {
        if node.borrow().left.is_none() && node.borrow().right.is_none() && sum == target_sum {
            return true;
        }
        if let Some(left) = &node.borrow().left {
            let sum = sum + left.borrow().val;
            if Self::traverse(left, sum, target_sum) {
                return true;
            }
        }
        if let Some(right) = &node.borrow().right {
            let sum = sum + right.borrow().val;
            if Self::traverse(right, sum, target_sum) {
                return true;
            }
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use super::TreeNode;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_case1() {
        let node30 = TreeNode::new(7);
        let node31 = TreeNode::new(2);
        let node32 = TreeNode::new(1);

        let mut node20 = TreeNode::new(11);
        node20.left = Some(Rc::new(RefCell::new(node30)));
        node20.right = Some(Rc::new(RefCell::new(node31)));
        let node21 = TreeNode::new(13);
        let mut node22 = TreeNode::new(4);
        node22.right = Some(Rc::new(RefCell::new(node32)));

        let mut node10 = TreeNode::new(4);
        node10.left = Some(Rc::new(RefCell::new(node20)));
        let mut node11 = TreeNode::new(8);
        node11.left = Some(Rc::new(RefCell::new(node21)));
        node11.right = Some(Rc::new(RefCell::new(node22)));

        let mut node00 = TreeNode::new(5);
        node00.left = Some(Rc::new(RefCell::new(node10)));
        node00.right = Some(Rc::new(RefCell::new(node11)));

        let root = Some(Rc::new(RefCell::new(node00)));
        let actual = Solution::has_path_sum(root, 22);
        assert_eq!(actual, true);
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

struct Solution;

fn main() {
    println!("Hello, world!");
}
