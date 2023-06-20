#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/second-minimum-node-in-a-binary-tree/
pub struct Solution;
use crate::util::i32_binary_tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> [i32; 2] {
            match root {
                None => [-1, -1],
                Some(root) => {
                    let (mut max1, mut max2) = (-1, -1);
                    let left = dfs(&root.borrow().left);
                    let right = dfs(&root.borrow().right);

                    for value in left
                        .into_iter()
                        .chain(right.into_iter())
                        .chain(vec![root.borrow().val])
                    {
                        if value > max1 {
                            max2 = max1;
                            max1 = value;
                        } else if value != max1 && value > max2 {
                            max2 = value;
                        }
                    }

                    [max1, max2]
                }
            }
        }
        
        dfs(&root)[1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            5,
            Solution::find_second_minimum_value(TreeNode::build(vec![2, 2, 5, -1, -1, 5, 7]))
        )
    }

    #[test]
    fn test_2() {
        assert_eq!(
            -1,
            Solution::find_second_minimum_value(TreeNode::build(vec![2, 2, 2]))
        )
    }
}
