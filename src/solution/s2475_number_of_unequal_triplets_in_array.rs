#![allow(dead_code)]

pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn unequal_triplets(nums: Vec<i32>) -> i32 {
        let counter: Vec<_> = nums
            .into_iter()
            .fold(HashMap::new(), |mut h, val| {
                *h.entry(val).or_insert(0) += 1;
                h
            })
            .values()
            .copied()
            .collect();
        let mut ans = 0;
        for i in 0..counter.len() {
            for j in i + 1..counter.len() {
                for k in j + 1..counter.len() {
                    ans += counter[i] * counter[j] * counter[k];
                }
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
        assert_eq!(3, Solution::unequal_triplets(vec![4, 4, 2, 4, 3]));
    }
    #[test]
    fn test_2() {
        assert_eq!(0, Solution::unequal_triplets(vec![1, 1, 1, 1, 1]));
    }
}
