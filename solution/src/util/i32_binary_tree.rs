use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn build(mut tree: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        // padding
        tree.insert(0, 0);
        Self::dfs(1, &tree)
    }

    fn dfs(u: usize, elements: &Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if u >= elements.len() || elements[u] == -1 {
            None
        } else {
            let root = Rc::new(RefCell::new(TreeNode::new(elements[u])));

            root.borrow_mut().left = Self::dfs(u << 1, elements);
            root.borrow_mut().right = Self::dfs(u << 1 | 1, elements);

            Some(root)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TreeNode;

    #[test]
    fn test_build() {
        let root = TreeNode::build(vec![2, 2, 5, -1, -1, 5, 7]);

        match root {
            None => panic!(),
            Some(root) => {
                assert_eq!(2, root.borrow().val);

                match &root.borrow().left {
                    None => panic!(),
                    Some(root) => {
                        assert_eq!(2, root.borrow().val);
                        assert!(root.borrow().left.is_none());
                        assert!(root.borrow().right.is_none());
                    }
                }

                match &root.borrow().right {
                    None => panic!(),
                    Some(root) => {
                        assert_eq!(5, root.borrow().val);

                        match &root.borrow().left {
                            None => panic!(),
                            Some(root) => {
                                assert_eq!(5, root.borrow().val);
                                assert!(root.borrow().left.is_none());
                                assert!(root.borrow().right.is_none());
                            }
                        }
                        match &root.borrow().right {
                            None => panic!(),
                            Some(root) => {
                                assert_eq!(7, root.borrow().val);
                                assert!(root.borrow().left.is_none());
                                assert!(root.borrow().right.is_none());
                            }
                        }
                    }
                }
            }
        }
    }
}
