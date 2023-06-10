#![allow(dead_code)]
pub struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let h : HashSet<_> = nums1.iter().collect();
        let mut ans = i32::MAX;
        
        for v in nums2.into_iter() {
            if v < ans && h.contains(&v) {
                ans = v;
            } 
        }

        if ans == i32::MAX {
            -1
        } else {
            ans
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::get_common(vec![1, 2, 3], vec![2, 4]));
    }

    #[test]
    fn test_2() {
        assert_eq!(2, Solution::get_common(vec![1, 2, 3, 6], vec![2, 3, 4, 5]));
    }

    #[test]
    fn test_3() {
        assert_eq!(-1, Solution::get_common(vec![1, 2, 3, 6], vec![7, 8, 9]));
    }
}
