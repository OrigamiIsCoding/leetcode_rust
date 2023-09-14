#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/construct-binary-tree-from-preorder-and-inorder-traversal/
pub struct Solution;

use crate::util::i32_binary_tree::TreeNode;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut h = HashMap::new();
        inorder.iter().copied().enumerate().for_each(|(i, v)| {
            h.insert(v, i);
        });

        fn build_tree_impl(
            preorder: &Vec<i32>,
            inorder: &Vec<i32>,
            pl: usize,
            pr: usize,
            il: usize,
            ir: usize,
            h: &HashMap<i32, usize>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if pl == pr {
                Some(Rc::new(RefCell::new(TreeNode::new(preorder[pl]))))
            } else if pl >= pr {
                None
            } else {
                let val = preorder[pl];
                let root = Rc::new(RefCell::new(TreeNode::new(val)));
                let k = h[&val];
                let l_count = k - il;
                root.borrow_mut().left =
                    build_tree_impl(preorder, inorder, pl + 1, pl + l_count, il, il + k - 1, h);
                root.borrow_mut().right =
                    build_tree_impl(preorder, inorder, pl + l_count + 1, pr, k + 1, ir, h);
                Some(root)
            }
        }

        build_tree_impl(
            &preorder,
            &inorder,
            0,
            preorder.len() - 1,
            0,
            inorder.len() - 1,
            &h,
        )
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
