/// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug, PartialOrd, Ord)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn box_list(value: Vec<i32>) -> Option<Box<ListNode>> {
        let mut sentinel = Box::new(ListNode::new(-1));
        let mut current = sentinel.as_mut();
        for val in value {
            current.next = Some(Box::new(ListNode::new(val)));
            current = current.next.as_mut().unwrap();
        }
        sentinel.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_list() {
        let actual = vec![1, 2, 3];
        let actual = ListNode::box_list(actual);
        let excepted = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        }));
        assert_eq!(actual, excepted);
    }
}
