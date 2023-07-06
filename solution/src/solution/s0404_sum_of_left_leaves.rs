#![allow(dead_code)]
use crate::util::i32_binary_tree::TreeNode;
/// reference: https://leetcode.cn/problems/sum-of-left-leaves/
pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root {
            root.borrow()
                .left
                .as_ref()
                .filter(|node| node.borrow().left.is_none() && node.borrow().right.is_none())
                .map(|node| node.borrow().val)
                .unwrap_or(0)
                + Self::sum_of_left_leaves(root.borrow().left.clone())
                + Self::sum_of_left_leaves(root.borrow().right.clone())
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree;

    #[test]
    fn test_1() {
        assert_eq!(
            24,
            Solution::sum_of_left_leaves(tree![3, 9, 20, null, null, 15, 7])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(0, Solution::sum_of_left_leaves(tree![1]));
    }

    #[test]
    fn test_3() {
        assert_eq!(4, Solution::sum_of_left_leaves(tree![1, 2, 3, 4, 5]));
    }

    #[test]
    fn test_4() {
        assert_eq!(
            5,
            Solution::sum_of_left_leaves(tree![0, 2, 4, 1, null, 3, -1, 5, 1, null, 6, null, 8])
        );
    }
}
