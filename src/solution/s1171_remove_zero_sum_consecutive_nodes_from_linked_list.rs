#![allow(dead_code)]
use crate::util::i32_singly_linked_list::ListNode;
pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut h = HashMap::new();

        let mut prefix = 0;
        let mut node = dummy.as_ref();
        while let Some(n) = node {
            prefix += n.val;
            h.insert(prefix, n);
            node = n.next.as_ref();
        }

        prefix = 0;
        let mut dummy = Box::new(ListNode::new(0));
        let mut node = Some(&mut dummy);
        while let Some(n) = node {
            prefix += n.val;
            if let Some(next) = h.get(&prefix) {
                n.next = match next.next.as_ref() {
                    Some(next) => Some(Box::new(ListNode::new(next.val))),
                    None => None,
                };
            }
            node = n.next.as_mut();
        }

        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;
    use crate::util::i32_singly_linked_list::ListNode;

    #[test]
    fn test_1() {
        let actual = BTreeSet::from_iter(vec![
            ListNode::box_list(vec![3, 1]),
            ListNode::box_list(vec![1, 2, 1]),
        ]);
        let excepted = Solution::remove_zero_sum_sublists(ListNode::box_list(vec![1, 2, -3, 3, 1]));
        assert!(actual.contains(&excepted));
    }

    #[test]
    fn test_2() {
        let actual = BTreeSet::from_iter(vec![ListNode::box_list(vec![1, 2, 4])]);
        let excepted = Solution::remove_zero_sum_sublists(ListNode::box_list(vec![1, 2, 3, -3, 4]));
        assert!(actual.contains(&excepted));
    }

    #[test]
    fn test_3() {
        let actual = BTreeSet::from_iter(vec![ListNode::box_list(vec![1])]);
        let excepted =
            Solution::remove_zero_sum_sublists(ListNode::box_list(vec![1, 2, 3, -3, -2]));
        assert!(actual.contains(&excepted));
    }
}
