#![allow(dead_code)]
use crate::util::i32_singly_linked_list::ListNode;
/// reference: https://leetcode.cn/problems/add-two-numbers/
pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut tail = &mut head;
        let mut t = 0;
        while l1.is_some() || l2.is_some() || t != 0 {
            if let Some(node) = l1 {
                t += node.val;
                l1 = node.next;
            }
            if let Some(node) = l2 {
                t += node.val;
                l2 = node.next;
            }
            *tail = Some(Box::new(ListNode::new(t % 10)));
            tail = &mut tail.as_mut().unwrap().next;
            t /= 10;
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use crate::list;

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            list![7, 0, 8],
            Solution::add_two_numbers(list![2, 4, 3], list![5, 6, 4])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(list![0], Solution::add_two_numbers(list![0], list![0]));
    }

    #[test]
    fn test_3() {
        assert_eq!(
            list![8, 9, 9, 9, 0, 0, 0, 1],
            Solution::add_two_numbers(list![9, 9, 9, 9, 9, 9, 9], list![9, 9, 9, 9])
        )
    }
}
