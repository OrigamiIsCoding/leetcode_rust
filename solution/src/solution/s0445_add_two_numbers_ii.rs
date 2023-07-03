#![allow(dead_code)]
use crate::util::i32_singly_linked_list::ListNode;
/// reference: https://leetcode.cn/problems/add-two-numbers-ii/
pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // head insertion
        let mut head = Box::new(ListNode::new(-1));
        let (mut stk1, mut stk2) = (Vec::new(), Vec::new());

        fn assign_into_stk(mut node: Option<Box<ListNode>>, stk: &mut Vec<i32>) {
            while let Some(n) = node {
                stk.push(n.val);
                node = n.next;
            }
        }

        assign_into_stk(l1, &mut stk1);
        assign_into_stk(l2, &mut stk2);

        let mut t = 0;
        while !stk1.is_empty() || !stk2.is_empty() || t != 0 {
            if let Some(val) = stk1.pop() {
                t += val;
            }
            if let Some(val) = stk2.pop() {
                t += val;
            }
            head.next = Some(Box::new(ListNode {
                val: t % 10,
                next: head.next.take(),
            }));
            t /= 10;
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
            list![7, 8, 0, 7],
            Solution::add_two_numbers(list![7, 2, 4, 3], list![5, 6, 4])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            list![8, 0, 7],
            Solution::add_two_numbers(list![2, 4, 3], list![5, 6, 4])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(list![0], Solution::add_two_numbers(list![0], list![0]));
    }
}
