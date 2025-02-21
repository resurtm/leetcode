use std::{cell::RefCell, collections::HashSet, rc::Rc};

struct FindElements {
    nums: HashSet<i32>,
}

impl FindElements {
    #[allow(dead_code)]
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut nums = HashSet::new();
        if let Some(x) = root {
            x.borrow_mut().val = 0;
            Self::traverse(&mut nums, x);
        }
        Self { nums }
    }

    #[allow(dead_code)]
    fn traverse(nums: &mut HashSet<i32>, node: Rc<RefCell<TreeNode>>) {
        let curr = node.borrow();
        nums.insert(curr.val);
        if let Some(x) = &curr.left {
            x.borrow_mut().val = 2 * curr.val + 1;
            Self::traverse(nums, Rc::clone(x));
        }
        if let Some(x) = &curr.right {
            x.borrow_mut().val = 2 * curr.val + 2;
            Self::traverse(nums, Rc::clone(x));
        }
    }

    #[allow(dead_code)]
    fn find(&self, target: i32) -> bool {
        self.nums.contains(&target)
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

fn main() {
    println!("nothing here");
}

#[cfg(test)]
mod tests {
    use super::{FindElements, TreeNode};
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn test_case1() {
        let root = Rc::new(RefCell::new(TreeNode::new(-1)));
        let level1 = Rc::new(RefCell::new(TreeNode::new(-1)));
        let level2 = Rc::new(RefCell::new(TreeNode::new(-1)));
        let level3 = Rc::new(RefCell::new(TreeNode::new(-1)));

        root.borrow_mut().right = Some(Rc::clone(&level1));
        level1.borrow_mut().left = Some(Rc::clone(&level2));
        level2.borrow_mut().left = Some(Rc::clone(&level3));

        let fe = FindElements::new(Some(root));
        assert!(fe.find(2));
        assert!(!fe.find(3));
        assert!(!fe.find(4));
        assert!(fe.find(5));
    }
}
