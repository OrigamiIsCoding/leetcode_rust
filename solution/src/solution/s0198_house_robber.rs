#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/house-robber/
pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let (mut a, mut b) = (0, 0);
        nums.into_iter().enumerate().for_each(|(i, v)| {
            if i % 2 == 0 {
                a = i32::max(a + v, b);
            } else {
                b = i32::max(b + v, a);
            }
        });
        i32::max(a, b)
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::rob(vec![1, 2, 3, 1]));
    }

    #[test]
    fn test_2() {
        assert_eq!(12, Solution::rob(vec![2, 7, 9, 3, 1]));
    }
}
