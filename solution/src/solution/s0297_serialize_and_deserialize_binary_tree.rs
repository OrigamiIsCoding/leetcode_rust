#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/serialize-and-deserialize-binary-tree/
pub struct Solution;
use crate::util::i32_binary_tree::TreeNode;

use std::cell::RefCell;
use std::iter::Peekable;
use std::rc::Rc;
use std::slice::Iter;

struct Codec {}

impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        fn serialize_impl(root: Option<Rc<RefCell<TreeNode>>>, result: &mut String) {
            match root {
                None => result.push_str("None "),
                Some(root) => {
                    result.push_str(&root.borrow().val.to_string());
                    result.push(' ');

                    serialize_impl(root.borrow_mut().left.take(), result);
                    serialize_impl(root.borrow_mut().right.take(), result);
                }
            }
        }

        let mut result = String::new();
        serialize_impl(root, &mut result);
        result
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        fn deserialize_impl(values: &mut Peekable<Iter<&str>>) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(&value) = values.next() {
                match value {
                    "None" => None,
                    _ => {
                        let mut root = TreeNode::new(value.parse().unwrap());

                        root.left = deserialize_impl(values);
                        root.right = deserialize_impl(values);

                        Some(Rc::new(RefCell::new(root)))
                    }
                }
            } else {
                None
            }
        }

        let values: Vec<_> = data.split_ascii_whitespace().collect();
        deserialize_impl(&mut values.iter().peekable())
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]

    use super::*;
    use crate::tree;

    #[test]
    fn test_1() {
        let codec = Codec::new();

        let serialized = codec.serialize(tree![1, 2, 3, null, null, 4, 5]);
        let deserialized = codec.deserialize(serialized);

        assert_eq!(tree![1, 2, 3, null, null, 4, 5], deserialized);
    }

    #[test]
    fn test_2() {
        let codec = Codec::new();
        let serialized = codec.serialize(tree![]);
        let deserialized = codec.deserialize(serialized);

        assert_eq!(tree![], deserialized);
    }

    #[test]
    fn test_3() {
        let codec = Codec::new();
        let serialized = codec.serialize(tree![1]);
        let deserialized = codec.deserialize(serialized);

        assert_eq!(tree![1], deserialized);
    }

    #[test]
    fn test_4() {
        let codec = Codec::new();
        let serialized = codec.serialize(tree![1, 2]);
        let deserialized = codec.deserialize(serialized);

        assert_eq!(tree![1, 2], deserialized);
    }
}
