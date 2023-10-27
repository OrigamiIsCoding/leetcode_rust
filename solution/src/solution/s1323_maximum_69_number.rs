#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/maximum-69-number/
pub struct Solution;

impl Solution {
    pub fn maximum69_number(num: i32) -> i32 {
        match (0..(num as f64).log10().ceil() as u32).rev().find(|&i| (num / 10_i32.pow(i)) % 10 == 6) {
            None => num,
            Some(i) => num + 10_i32.pow(i) * 3,
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(9969, Solution::maximum69_number(9669));
    }

    #[test]
    fn test_2() {
        assert_eq!(9999, Solution::maximum69_number(9996));
    }

    #[test]
    fn test_3() {
        assert_eq!(9999, Solution::maximum69_number(9999));
    }
}
