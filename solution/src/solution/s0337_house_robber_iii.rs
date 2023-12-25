#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/house-robber-iii/
pub struct Solution;
use crate::util::i32_binary_tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn rob_impl(root: Option<Rc<RefCell<TreeNode>>>) -> [i32; 2] {
            match root {
                None => [0, 0],
                Some(root) => {
                    let left = rob_impl(root.borrow_mut().left.take());
                    let right = rob_impl(root.borrow_mut().right.take());
                    [
                        i32::max(root.borrow().val + left[1] + right[1], left[0] + right[0]),
                        left[0] + right[0],
                    ]
                }
            }
        }
        rob_impl(root).into_iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]

    use super::*;
    use crate::tree;

    #[test]
    fn test_1() {
        assert_eq!(7, Solution::rob(tree![3, 2, 3, null, 3, null, 1]));
    }

    #[test]
    fn test_2() {
        assert_eq!(9, Solution::rob(tree![3, 4, 5, 1, 3, null, 1]));
    }
}
