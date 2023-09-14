#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/construct-binary-tree-from-inorder-and-postorder-traversal/
pub struct Solution;
use crate::util::i32_binary_tree::TreeNode;

use std::cell::RefCell;
use std::iter::{Peekable, Rev};
use std::rc::Rc;
use std::slice::Iter;

impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build_tree_impl(
            inorder: &[i32],
            postorder: &mut Peekable<Rev<Iter<i32>>>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if inorder.is_empty() {
                return None;
            }

            let val = postorder.next().copied().unwrap();
            let root = Rc::new(RefCell::new(TreeNode::new(val)));

            let idx = inorder
                .iter()
                .enumerate()
                .find(|(_, &v)| v == val)
                .map(|(i, _)| i)
                .unwrap();

            root.borrow_mut().right = build_tree_impl(&inorder[idx + 1..], postorder);
            root.borrow_mut().left = build_tree_impl(&inorder[0..idx], postorder);

            Some(root)
        }

        build_tree_impl(&inorder, &mut postorder.iter().rev().peekable())
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]

    use super::*;
    use crate::tree;

    #[test]
    fn test_1() {
        assert_eq!(
            tree![3, 9, 20, null, null, 15, 7],
            Solution::build_tree(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(tree![-1], Solution::build_tree(vec![-1], vec![-1]));
    }
}
