#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/minimum-operations-to-make-the-array-increasing/
pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        nums.into_iter().fold(0, |mut acc, v| {
            acc += i32::max(0, (max + 1) - v);
            max = i32::max(max + 1, v);
            acc
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::min_operations(vec![1, 1, 1]));
    }

    #[test]
    fn test_2() {
        assert_eq!(14, Solution::min_operations(vec![1, 5, 2, 4, 1]));
    }

    #[test]
    fn test_3() {
        assert_eq!(0, Solution::min_operations(vec![8]));
    }
}
