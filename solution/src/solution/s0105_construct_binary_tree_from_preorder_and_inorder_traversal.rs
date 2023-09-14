#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/construct-binary-tree-from-preorder-and-inorder-traversal/
pub struct Solution;

use crate::util::i32_binary_tree::TreeNode;

use std::cell::RefCell;
use std::iter::Peekable;
use std::rc::Rc;
use std::slice::Iter;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build_tree_impl(
            preorder: &mut Peekable<Iter<i32>>,
            inorder: &[i32],
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if inorder.is_empty() {
                return None;
            }

            let val = preorder.next().copied().unwrap();
            let root = Rc::new(RefCell::new(TreeNode::new(val)));

            let idx = inorder
                .iter()
                .enumerate()
                .find(|(_, &v)| v == val)
                .map(|(i, _)| i)
                .unwrap();

            root.borrow_mut().left = build_tree_impl(preorder, &inorder[0..idx]);
            root.borrow_mut().right = build_tree_impl(preorder, &inorder[idx + 1..]);
            Some(root)
        }

        build_tree_impl(&mut preorder.iter().peekable(), &inorder)
    }
}

#[cfg(test)]
mod tests {
    use crate::tree;

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            tree![3, 9, 20, null, null, 15, 7],
            Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7])
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(tree![1, 2], Solution::build_tree(vec![1, 2], vec![2, 1]));
    }
}
