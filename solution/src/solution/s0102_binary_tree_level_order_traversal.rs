#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/binary-tree-level-order-traversal/
pub struct Solution;
use crate::util::i32_binary_tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();

        let mut queue = std::collections::VecDeque::new();
        let (mut nodes, mut current) = (Vec::new(), 0);
        queue.push_back((root, 0));
        while let Some((node, level)) = queue.pop_front() {
            if let Some(node) = node {
                queue.push_back((node.borrow_mut().left.take(), level + 1));
                queue.push_back((node.borrow_mut().right.take(), level + 1));
                if level == current {
                    nodes.push(node.borrow().val);
                } else {
                    current += 1;
                    ans.push(nodes);
                    nodes = vec![node.borrow().val];
                }
            }
        }

        if !nodes.is_empty() {
            ans.push(nodes);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::{mat, tree};

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            mat![[3], [9, 20], [15, 7]],
            Solution::level_order(tree![3, 9, 20, null, null, 15, 7])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(mat![[1]], Solution::level_order(tree![1]));
    }

    #[test]
    fn test_3() {
        assert_eq!(mat![] as Vec<Vec<i32>>, Solution::level_order(tree![]));
    }
}
