/// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
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

#[macro_export]
macro_rules! list {
    ( $( $ele:expr ) , *) => {
       ListNode::box_list(vec![$($ele.into()), *])
    };
}

#[macro_export]
macro_rules! list_vec {
    [ $( [ $( $d:expr ),* ] ),* ] => {
        vec![
            $(
                list![$($d),*],
            )*
        ]
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
        assert_eq!(excepted, actual);
    }

    #[test]
    fn test_list() {
        let excepted = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        }));
        assert_eq!(excepted, list![1, 2, 3]);
    }

    #[test]
    fn test_list_vec() {
        let excepted = vec![list![3, 2, 1], list![2, 1], list![1], list![]];
        let acutal = list_vec![[3, 2, 1], [2, 1], [1], []];
        assert_eq!(excepted, acutal);
    }
}
