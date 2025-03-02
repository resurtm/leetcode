impl Solution {
    #[allow(dead_code)]
    pub fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut curr = Self::reverse_list(head);
        let mut res: Option<Box<ListNode>> = None;
        let mut last = i32::MIN;
        while let Some(mut node) = curr {
            curr = node.next.take();
            if node.val >= last {
                res = Some(Box::new(ListNode {
                    val: node.val,
                    next: res,
                }));
                last = node.val;
            }
        }
        res
    }

    fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut prev, mut curr) = (None, head);
        while let Some(mut node) = curr {
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            curr = next;
        }
        prev
    }
}

struct Solution {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[allow(dead_code)]
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn main() {
    println!("nothing here");
}

#[cfg(test)]
mod tests {
    use super::{ListNode, Solution};

    #[test]
    fn test_case1() {
        let actual = Solution::remove_nodes(Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 13,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode::new(8))),
                    })),
                })),
            })),
        })));
        let expected = Some(Box::new(ListNode {
            val: 13,
            next: Some(Box::new(ListNode::new(8))),
        }));
        assert_eq!(actual, expected);
    }
}
