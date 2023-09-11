#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/add-two-integers/
pub struct Solution;

impl Solution {
    pub fn sum(num1: i32, num2: i32) -> i32 {
        num1 + num2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(17, Solution::sum(12, 5));
    }

    #[test]
    fn test_2() {
        assert_eq!(-6, Solution::sum(-10, 4));
    }
}
