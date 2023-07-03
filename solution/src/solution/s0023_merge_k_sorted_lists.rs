#![allow(dead_code)]
use crate::util::i32_singly_linked_list::ListNode;
/// reference: https://leetcode.cn/problems/merge-k-sorted-lists/
pub struct Solution;

use std::{
    cmp::{Ord, Ordering, PartialOrd},
    collections::BinaryHeap,
};

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut tail = &mut head;
        let mut heap = BinaryHeap::new();

        lists.into_iter().flatten().for_each(|node| {
            heap.push(node);
        });

        while let Some(mut node) = heap.pop() {
            if let Some(node) = node.next.take() {
                heap.push(node);
            }
            tail = &mut tail.insert(node).next;
        }

        head
    }
}

#[cfg(test)]
mod tests {
    use crate::{list, list_vec};

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            list![1, 1, 2, 3, 4, 4, 5, 6],
            Solution::merge_k_lists(list_vec![[1, 4, 5], [1, 3, 4], [2, 6]])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(list![], Solution::merge_k_lists(list_vec![]));
    }

    #[test]
    fn test_3() {
        assert_eq!(list![], Solution::merge_k_lists(list_vec![[]]));
    }
}
