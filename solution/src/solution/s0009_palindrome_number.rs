#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/palindrome-number/
pub struct Solution;

impl Solution {
    pub fn is_palindrome(mut x: i32) -> bool {
        if x < 0 {
            false
        } else {
            let mut deque = std::collections::VecDeque::new();
            while x != 0 {
                let t = x % 10;
                x /= 10;
                deque.push_back(t);
            }
            while !deque.is_empty() {
                match (deque.pop_front(), deque.pop_back()) {
                    (Some(l), Some(r)) if l != r => return false,
                    _ => (),
                }
            }
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::is_palindrome(121));
    }

    #[test]
    fn test_2() {
        assert_eq!(false, Solution::is_palindrome(-121));
    }

    #[test]
    fn test_3() {
        assert_eq!(false, Solution::is_palindrome(10));
    }

    #[test]
    fn test_4() {
        assert_eq!(false, Solution::is_palindrome(1000021));
    }
}
