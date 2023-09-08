#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/longest-even-odd-subarray-with-threshold/
pub struct Solution;

impl Solution {
    pub fn longest_alternating_subarray(nums: Vec<i32>, threshold: i32) -> i32 {
        let mut nums = nums.into_iter();
        let (mut prev, mut ans, mut current) = (None, 0, 0);
        while let Some(value) = nums.next() {
            if value > threshold {
                prev = None;
                continue;
            }
            prev = if prev.map_or(false, |prev| prev % 2 != value % 2) {
                current += 1;
                Some(value)
            } else {
                if value % 2 == 0 {
                    current = 1;
                    Some(value)
                } else {
                    current = 0;
                    None
                }
            };
            ans = ans.max(current);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            3,
            Solution::longest_alternating_subarray(vec![3, 2, 5, 4], 5)
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(1, Solution::longest_alternating_subarray(vec![1, 2], 2));
    }

    #[test]
    fn test_3() {
        assert_eq!(
            3,
            Solution::longest_alternating_subarray(vec![2, 3, 4, 5], 4)
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(1, Solution::longest_alternating_subarray(vec![2], 4));
    }

    #[test]
    fn test_5() {
        assert_eq!(0, Solution::longest_alternating_subarray(vec![1], 1));
    }
}
