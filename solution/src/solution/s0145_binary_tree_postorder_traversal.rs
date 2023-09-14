#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/binary-tree-postorder-traversal/
pub struct Solution;
use crate::util::i32_binary_tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn postorder_traversal_impl(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
            if let Some(root) = root {
                postorder_traversal_impl(root.borrow_mut().left.take(), ans);
                postorder_traversal_impl(root.borrow_mut().right.take(), ans);
                ans.push(root.borrow().val);
            }
        }

        let mut ans = Vec::new();
        postorder_traversal_impl(root, &mut ans);
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::tree;

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![3, 2, 1],
            Solution::postorder_traversal(tree![1, null, 2, 3])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(vec![0; 0], Solution::postorder_traversal(tree![]));
    }

    #[test]
    fn test_3() {
        assert_eq!(vec![1], Solution::postorder_traversal(tree![1]));
    }
}
