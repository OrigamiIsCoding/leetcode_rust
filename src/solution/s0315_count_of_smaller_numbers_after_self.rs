#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/count-of-smaller-numbers-after-self/
pub struct Solution;

use crate::data_struct_impl::FenwickTree;

impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        const OFFSET: usize = 10_001;
        let mut ft = FenwickTree::new(OFFSET * 2);
        let mut ans = Vec::new();

        for pos in nums
            .into_iter()
            .map(|num| (num + OFFSET as i32) as usize)
            .rev()
        {
            let result = ft.query(pos - 1);
            ft.add(pos, 1);
            ans.push(result);
        }
        ans.reverse();
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![2, 1, 1, 0], Solution::count_smaller(vec![5, 2, 6, 1]));
    }

    #[test]
    fn test_2() {
        assert_eq!(vec![0], Solution::count_smaller(vec![-1]));
    }

    #[test]
    fn test_3() {
        assert_eq!(vec![0, 0], Solution::count_smaller(vec![-1, -1]));
    }
}
