#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/largest-rectangle-in-histogra/
pub struct Solution;

struct MonotoneStack<T>
where
    T: Copy,
{
    container: Vec<T>,
    cmp: Box<dyn Fn(T, T) -> bool>,
}

// impl<T> MonotoneStack<T>
// where
//     T: Copy,
// {
//     fn new(cmp: impl Fn(T, T) -> bool) -> Self {
//         Self {
//             container: Vec::new(),
//             cmp: Box::new(cmp),
//         }
//     }

//     fn top(&self) -> Option<&T> {
//         self.container.last()
//     }

//     fn push(&mut self, value: T) {
//         while let Some(&top) = self.top() {
//             if (self.cmp)(top, value) {
//                 self.pop();
//             } else {
//                 break;
//             }
//         }
//         self.container.push(value);
//     }

//     fn pop(&mut self) -> Option<T> {
//         self.container.pop()
//     }

//     fn len(&self) -> usize {
//         self.container.len()
//     }
// }

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        // let mut stack = MonotoneStack::new(|&a, &b| heights[a] < heights[b]);
        // let mut ans = 0;
        // for (i, h) in heights.iter().enumerate() {}
        // ans
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(10, Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]));
    }

    #[test]
    fn test_2() {
        assert_eq!(4, Solution::largest_rectangle_area(vec![2, 4]));
    }

    #[test]
    fn test_3() {
        assert_eq!(2, Solution::largest_rectangle_area(vec![2, 0, 2]));
    }
}
