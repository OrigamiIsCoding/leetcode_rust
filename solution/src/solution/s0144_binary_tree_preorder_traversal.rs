#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/binary-tree-preorder-traversal/
pub struct Solution;
use crate::util::i32_binary_tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = Vec::new();
        Self::preorder_traversal_impl(root, &mut ans);
        ans
    }

    fn preorder_traversal_impl(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
        if let Some(root) = root {
            ans.push(root.borrow().val);
            Self::preorder_traversal_impl(root.borrow_mut().left.take(), ans);
            Self::preorder_traversal_impl(root.borrow_mut().right.take(), ans);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tree;

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![1, 2, 3],
            Solution::preorder_traversal(tree![1, null, 2, 3])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(vec![0; 0], Solution::preorder_traversal(tree![]))
    }

    #[test]
    fn test_3() {
        assert_eq!(vec![1], Solution::preorder_traversal(tree![1]))
    }
}
