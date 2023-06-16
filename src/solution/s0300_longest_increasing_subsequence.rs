#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/longest-increasing-subsequence/
pub struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut f = vec![0; 1];
        for num in nums {
            let (mut l, mut r) = (0, f.len() - 1);
            while l < r {
                let mid = (l + r + 1) / 2;
                if f[mid] >= num {
                    r = mid - 1;
                } else {
                    l = mid;
                }
            }
            match f.get_mut(l + 1) {
                Some(value) => *value = num,
                None => f.push(num),
            }
        }
        (f.len() - 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]));
    }

    #[test]
    fn test_2() {
        assert_eq!(4, Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]));
    }

    #[test]
    fn test_3() {
        assert_eq!(1, Solution::length_of_lis(vec![7; 10]));
    }
}
