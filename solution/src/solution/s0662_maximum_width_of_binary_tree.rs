#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/maximum-width-of-binary-tree/
pub struct Solution;
use crate::util::i32_binary_tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (mut min, mut max) = (Vec::new(), Vec::new());

        fn dfs(
            deep: usize,
            index: u32,
            root: &Option<Rc<RefCell<TreeNode>>>,
            min: &mut Vec<u32>,
            max: &mut Vec<u32>,
        ) {
            if let Some(root) = root {
                let root = root.borrow();
                match min.get_mut(deep) {
                    None => min.push(index),
                    Some(v) => *v = index.min(*v),
                }
                match max.get_mut(deep) {
                    None => max.push(index),
                    Some(v) => *v = index.max(*v),
                }
                dfs(deep + 1, index << 1, &root.left, min, max);
                dfs(deep + 1, index << 1 | 1, &root.right, min, max);
            }
        }

        dfs(0, 1, &root, &mut min, &mut max);

        min.into_iter()
            .zip(max.into_iter())
            .map(|(a, b)| b - a + 1)
            .max()
            .unwrap_or(0) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            4,
            Solution::width_of_binary_tree(TreeNode::build(vec![1, 3, 2, 5, 3, -1, 9]))
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            7,
            Solution::width_of_binary_tree(TreeNode::build(vec![1, 3, 2, 5, -1, -1, 9, 6, -1, 7]))
        );
    }
    #[test]
    fn test_3() {
        assert_eq!(
            2,
            Solution::width_of_binary_tree(TreeNode::build(vec![1, 3, 2, 5]))
        );
    }
}
