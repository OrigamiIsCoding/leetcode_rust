#![allow(dead_code)]
use std::cell::RefCell;
use std::fmt::Debug;
/// reference: https://leetcode.cn/problems/design-linked-list/
use std::rc::Rc;
struct ListNode {
    value: i32,
    next: Option<Rc<RefCell<ListNode>>>,
    prev: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    #[inline]
    fn new(value: i32) -> Self {
        Self {
            value,
            next: None,
            prev: None,
        }
    }
}

struct MyLinkedList {
    head: Rc<RefCell<ListNode>>,
    tail: Rc<RefCell<ListNode>>,
    size: usize,
}

#[inline]
fn new_node(value: i32) -> Rc<RefCell<ListNode>> {
    Rc::new(RefCell::new(ListNode::new(value)))
}

impl Debug for MyLinkedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut values = Vec::new();

        let mut current = self.head.borrow().next.clone();
        while if let Some(current) = current {
            if current == self.tail {
                break
            }
            values.push(current.borrow().val);
            current = current.borrow().next.unwrap();
        }

        f.debug_struct("MyLinkedList")
            .field("values", &values)
            .field("size", &self.size)
            .finish()
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyLinkedList {
    fn new() -> Self {
        let head = new_node(-1);
        let tail = new_node(-1);
        head.borrow_mut().next = Some(Rc::clone(&tail));
        tail.borrow_mut().prev = Some(Rc::clone(&head));
        Self {
            head,
            tail,
            size: 0,
        }
    }

    fn get_head_next(&self, index: i32) -> Rc<RefCell<ListNode>> {
        let mut current = Rc::clone(&self.head);
        for _ in 0..index {
            current = current.clone().borrow().next.clone().unwrap();
        }
        current
    }

    fn get(&self, index: i32) -> i32 {
        if index as usize >= self.size {
            -1
        } else {
            self.get_head_next(index).borrow().value
        }
    }

    fn add_at_head(&mut self, val: i32) {
        self.add_at_index(0, val);
    }

    fn add_at_tail(&mut self, val: i32) {
        self.add_at_index(self.size as i32 + 1, val);
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        if index as usize > self.size {
            return;
        }
        let prev = self.get_head_next(index);
        let node = new_node(val);
        let next = prev.borrow().next.clone().unwrap();

        node.borrow_mut().next = Some(Rc::clone(&next));
        node.borrow_mut().prev = Some(Rc::clone(&prev));
        next.borrow_mut().prev = Some(Rc::clone(&node));
        prev.borrow_mut().next = Some(Rc::clone(&node));
        self.size += 1;
    }

    fn delete_at_index(&self, index: i32) {
        let node = self.get_head_next(index);
        let prev = node.borrow().prev.clone().unwrap();
        let next = node.borrow().next.clone().unwrap();

        prev.borrow_mut().next = Some(Rc::clone(&next));
        next.borrow_mut().prev = Some(Rc::clone(&prev));
    }
}

/**
 * Your MyLinkedList object will be instantiated and called as such:
 * let obj = MyLinkedList::new();
 * let ret_1: i32 = obj.get(index);
 * obj.add_at_head(val);
 * obj.add_at_tail(val);
 * obj.add_at_index(index, val);
 * obj.delete_at_index(index);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut list = MyLinkedList::new();
        list.add_at_head(1);
        list.add_at_tail(3);
        list.add_at_index(1, 2);
        assert_eq!(2, list.get(1));
        list.delete_at_index(1);
        assert_eq!(3, list.get(1));
    }
}
