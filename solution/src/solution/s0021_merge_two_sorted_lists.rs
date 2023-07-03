#![allow(dead_code)]
use crate::util::i32_singly_linked_list::ListNode;
/// reference: https://leetcode.cn/problems/merge-two-sorted-lists/
pub struct Solution;

impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut tail = &mut head;
        while let (Some(l1), Some(l2)) = (list1.as_ref(), list2.as_ref()) {
            let l = if l1.val <= l2.val {
                &mut list1
            } else {
                &mut list2
            };
            *tail = l.take();
            *l = tail.as_mut().unwrap().next.take();
            tail = &mut tail.as_mut().unwrap().next;
        }
        *tail = list1.or(list2);
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
            list![1, 1, 2, 3, 4, 4],
            Solution::merge_two_lists(list![1, 2, 4], list![1, 3, 4])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(list![], Solution::merge_two_lists(list![], list![]));
    }

    #[test]
    fn test_3() {
        assert_eq!(list![0], Solution::merge_two_lists(list![], list![0]));
    }
}
