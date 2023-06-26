#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/maximum-value-of-a-string-in-an-array/
pub struct Solution;

impl Solution {
    pub fn maximum_value(strs: Vec<String>) -> i32 {
        strs.into_iter()
            .map(|s| s.parse::<i32>().unwrap_or(s.len() as i32))
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::vec_into;

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            5,
            Solution::maximum_value(vec_into!["alic3", "bob", "3", "4", "00000"])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            1,
            Solution::maximum_value(vec_into!["1", "01", "001", "0001"])
        );
    }
}
