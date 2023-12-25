#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/tuple-with-same-product/
pub struct Solution;

impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        let mut h = std::collections::HashMap::new();
        for (idx, &i) in nums.iter().enumerate() {
            for &j in nums.iter().skip(idx + 1) {
                *h.entry(i * j).or_insert(0) += 1;
            }
        }
        h.values().map(|&v| v * (v - 1) * 4).sum()
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(8, Solution::tuple_same_product(vec![2, 3, 4, 6]));
    }

    #[test]
    fn test_2() {
        assert_eq!(16, Solution::tuple_same_product(vec![1, 2, 4, 5, 10]));
    }
}
