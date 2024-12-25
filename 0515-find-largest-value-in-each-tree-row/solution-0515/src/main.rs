use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];

        let mut queue = vec![];
        if let Some(root_ref) = root {
            queue.push(root_ref);
        }

        while !queue.is_empty() {
            let mut max: Option<i32> = None;
            let mut next = vec![];

            while !queue.is_empty() {
                let curr_ref = queue.pop().unwrap();
                let curr_node = curr_ref.borrow();
                if max.is_none() || max.unwrap() < curr_node.val {
                    max = Some(curr_node.val);
                }
                if let Some(left_ref) = &curr_node.left {
                    next.push(Rc::clone(left_ref));
                }
                if let Some(right_ref) = &curr_node.right {
                    next.push(Rc::clone(right_ref));
                }
            }

            res.push(max.unwrap());
            queue = next;
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
    use super::{Solution, TreeNode};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_case1() {
        let root = Rc::new(RefCell::new(TreeNode::new(1)));

        let node00 = Rc::new(RefCell::new(TreeNode::new(3)));
        let node01 = Rc::new(RefCell::new(TreeNode::new(2)));

        let node10 = Rc::new(RefCell::new(TreeNode::new(5)));
        let node11 = Rc::new(RefCell::new(TreeNode::new(3)));
        let node12 = Rc::new(RefCell::new(TreeNode::new(9)));

        root.borrow_mut().left = Some(Rc::clone(&node00));
        root.borrow_mut().right = Some(Rc::clone(&node01));

        node00.borrow_mut().left = Some(Rc::clone(&node10));
        node00.borrow_mut().right = Some(Rc::clone(&node11));

        node01.borrow_mut().left = Some(Rc::clone(&node12));

        let actual = Solution::largest_values(Some(Rc::clone(&root)));
        assert_eq!(actual, vec![1, 3, 9]);
    }
}

struct Solution;

fn main() {
    println!("Hello, world!");
}
