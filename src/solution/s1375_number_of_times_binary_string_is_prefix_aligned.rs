#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/number-of-times-binary-string-is-prefix-aligned/
pub struct Solution;

use crate::data_struct_impl::FenwickTree;

impl Solution {
    pub fn num_times_all_blue(flips: Vec<i32>) -> i32 {
        let mut ft = FenwickTree::new(flips.len());
        let mut ans = 0;
        for flip in flips {
            ft.add(flip as usize, 1);
            let count = ft.query(ft.len());
            if count == ft.query(count) {
                ans += 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::num_times_all_blue(vec![3, 2, 4, 1, 5]));
    }

    #[test]
    fn test_2() {
        assert_eq!(1, Solution::num_times_all_blue(vec![4, 1, 2, 3]));
    }
}
