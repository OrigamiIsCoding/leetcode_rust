#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/two-sum/
pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut h = std::collections::HashMap::new();
        for (index, val) in nums.into_iter().enumerate() {
            if let Some(&idx) = h.get(&(target - val)) {
                return vec![idx, index as i32];
            } else {
                h.insert(val, index as i32);
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!([vec![0, 1], vec![1, 0]].contains(&Solution::two_sum(vec![2, 7, 11, 15], 9)));
    }

    #[test]
    fn test_2() {
        assert!([vec![1, 2], vec![2, 1]].contains(&Solution::two_sum(vec![3, 2, 4], 6)));
    }

    #[test]
    fn test_3() {
        assert!([vec![0, 1], vec![1, 0]].contains(&Solution::two_sum(vec![3, 3], 6)));
    }
}
