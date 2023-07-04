#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/k-items-with-the-maximum-sum/
pub struct Solution;

impl Solution {
    pub fn k_items_with_maximum_sum(
        num_ones: i32,
        num_zeros: i32,
        _num_neg_ones: i32,
        k: i32,
    ) -> i32 {
        match k {
            _ if k <= num_ones => k,
            _ if k <= num_ones + num_zeros => num_ones,
            _ => 2 * num_ones + num_zeros - k,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::k_items_with_maximum_sum(3, 2, 0, 2));
    }

    #[test]
    fn test_2() {
        assert_eq!(3, Solution::k_items_with_maximum_sum(3, 2, 0, 4));
    }

    #[test]
    fn test_3() {
        assert_eq!(5, Solution::k_items_with_maximum_sum(6, 6, 6, 13));
    }
}
