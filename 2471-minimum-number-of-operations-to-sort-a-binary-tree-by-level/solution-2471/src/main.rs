use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn minimum_operations(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        let mut level: Vec<Rc<RefCell<TreeNode>>> = vec![root.unwrap()];

        loop {
            let mut new_level: Vec<Rc<RefCell<TreeNode>>> = vec![];
            for item in level.iter() {
                let item_node = item.borrow();
                if let Some(left_ref) = &item_node.left {
                    new_level.push(Rc::clone(left_ref));
                }
                if let Some(right_ref) = &item_node.right {
                    new_level.push(Rc::clone(right_ref));
                }
            }
            if new_level.is_empty() {
                break;
            }

            let mut items: Vec<_> = new_level
                .iter()
                .enumerate()
                .map(|(i, v)| (i, v.borrow().val))
                .collect();
            items.sort_by(|x, y| x.1.partial_cmp(&y.1).unwrap());

            let mut viz = Vec::new();
            viz.resize(items.len(), false);

            for &(i, it) in items.iter() {
                if viz[i] || items[i].1 == it {
                    continue;
                }
                let mut curr_res = 0;
                let mut j = i;
                while !viz[j] {
                    viz[j] = true;
                    j = items[j].0;
                    curr_res += 1;
                }
                if curr_res > 0 {
                    res += curr_res - 1;
                }
            }

            level = new_level;
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

        let node00 = Rc::new(RefCell::new(TreeNode::new(4)));
        let node01 = Rc::new(RefCell::new(TreeNode::new(3)));

        let node10 = Rc::new(RefCell::new(TreeNode::new(7)));
        let node11 = Rc::new(RefCell::new(TreeNode::new(6)));
        let node12 = Rc::new(RefCell::new(TreeNode::new(8)));
        let node13 = Rc::new(RefCell::new(TreeNode::new(5)));

        let node20 = Rc::new(RefCell::new(TreeNode::new(9)));
        let node21 = Rc::new(RefCell::new(TreeNode::new(10)));

        root.borrow_mut().left = Some(Rc::clone(&node00));
        root.borrow_mut().right = Some(Rc::clone(&node01));

        node00.borrow_mut().left = Some(Rc::clone(&node10));
        node00.borrow_mut().right = Some(Rc::clone(&node11));

        node01.borrow_mut().left = Some(Rc::clone(&node12));
        node01.borrow_mut().right = Some(Rc::clone(&node13));

        node12.borrow_mut().left = Some(Rc::clone(&node20));

        node13.borrow_mut().left = Some(Rc::clone(&node21));

        let actual = Solution::minimum_operations(Some(Rc::clone(&root)));
        assert_eq!(actual, 3);
    }
}

struct Solution;

fn main() {
    println!("Hello, world!");
}
