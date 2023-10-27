#![allow(dead_code)]

/// reference: https://leetcode.cn/problems/partition-array-such-that-maximum-difference-is-k/
pub struct Solution;

impl Solution {
    pub fn partition_array(nums: Vec<i32>, k: i32) -> i32 {
        let max_value = nums.iter().max().copied().unwrap() as usize;
        let counter = nums.into_iter().fold(vec![0; max_value + 1], |mut acc, v| {
            acc[v as usize] += 1;
            acc
        });
        let (mut i, mut ans) = (0, 0);
        while i <= max_value {
            if counter[i] > 0 {
                ans += 1;
                i += k as usize + 1;
            } else {
                i += 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::partition_array(vec![3, 6, 1, 2, 5], 2));
    }

    #[test]
    fn test_2() {
        assert_eq!(2, Solution::partition_array(vec![1, 2, 3], 1));
    }

    #[test]
    fn test_3() {
        assert_eq!(3, Solution::partition_array(vec![2, 2, 4, 5], 0));
    }
}
