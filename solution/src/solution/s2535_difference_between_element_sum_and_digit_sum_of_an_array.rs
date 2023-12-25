#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/difference-between-element-sum-and-digit-sum-of-an-array/
pub struct Solution;

impl Solution {
    pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
        let result = nums
            .into_iter()
            .fold((0, 0), |(mut element_sum, mut digit_sum), mut v| {
                element_sum += v;
                while v != 0 {
                    digit_sum += v % 10;
                    v /= 10;
                }
                (element_sum, digit_sum)
            });
        i32::abs(result.0 - result.1)
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(9, Solution::difference_of_sum(vec![1, 15, 6, 3]));
    }

    #[test]
    fn test_2() {
        assert_eq!(0, Solution::difference_of_sum(vec![1, 2, 3, 4]));
    }
}
