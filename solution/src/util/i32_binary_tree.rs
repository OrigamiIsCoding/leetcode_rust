use std::cell::RefCell;
use std::collections::VecDeque;
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

    pub fn build(tree: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if tree.is_empty() {
            return None;
        }
        let nodes: Vec<_> = tree
            .iter()
            .map(|val| val.map(|val| Rc::new(RefCell::new(TreeNode::new(val)))))
            .collect();
        let mut tree = tree
            .into_iter()
            .enumerate()
            .map(|(idx, val)| val.map(|_| idx));
        let root = nodes.first().unwrap().clone();

        let mut queue = VecDeque::new();
        queue.push_back(tree.next().flatten().unwrap());

        while let Some(front) = queue.pop_front() {
            let mut node = nodes[front].as_ref().unwrap().borrow_mut();

            if let Some(Some(left)) = tree.next() {
                node.left = Some(nodes[left].as_ref().unwrap().clone());
                queue.push_back(left);
            }
            if let Some(Some(right)) = tree.next() {
                node.right = Some(nodes[right].as_ref().unwrap().clone());
                queue.push_back(right);
            }
        }

        root
    }
}

#[macro_export]
macro_rules! tree {
    ($($e:expr),*) => {
        {
            let s = stringify!($($e),*);
            if s.trim().is_empty() {
                None
            } else {
                let v: Vec<Option<i32>> = s.split(',')
                    .map(|x| x.trim())
                    .map(|x| match x {
                        "null" => None,
                        _ => Some(x.parse().unwrap())
                    })
                    .collect();
                TreeNode::build(v)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::TreeNode;

    #[test]
    fn test_build() {
        let root = TreeNode::build(vec![
            Some(2),
            Some(2),
            Some(5),
            None,
            None,
            Some(5),
            Some(7),
        ]);

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

    #[test]
    fn test_macro_tree() {
        assert_eq!(None, tree![]);
        let root = tree![2, 2, 5, null, null, 5, 7];
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
