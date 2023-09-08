#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/sum-lists-lcci/
use crate::util::i32_singly_linked_list::ListNode;
pub struct Solution;
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = ListNode::new(-1);
        let mut tail = &mut head;
        let mut r = (l1, l2);
        let mut t = 0;

        loop {
            r = match r {
                (None, None) => break,
                (None, Some(mut node)) => {
                    t += node.val;
                    (None, node.next.take())
                }
                (Some(mut node), None) => {
                    t += node.val;
                    (node.next.take(), None)
                }
                (Some(mut n1), Some(mut n2)) => {
                    t += n1.val + n2.val;
                    (n1.next.take(), n2.next.take())
                }
            };
            tail.next = Some(Box::new(ListNode::new(t % 10)));
            tail = tail.next.as_mut().unwrap();
            t /= 10;
        }

        if t != 0 {
            tail.next = Some(Box::new(ListNode::new(t)));
        }

        head.next
    }
}

#[cfg(test)]
mod tests {
    use crate::list;

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            list![2, 1, 9],
            Solution::add_two_numbers(list![7, 1, 6], list![5, 9, 2])
        );
    }
}
