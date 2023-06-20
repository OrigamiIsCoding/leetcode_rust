#![allow(dead_code)]

/// reference: https://leetcode.cn/problems/greatest-sum-divisible-by-three/
pub struct Solution;

use std::collections::BTreeSet;
impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        let mut result = [BTreeSet::new(), BTreeSet::new(), BTreeSet::new()];

        for num in nums {
            let idx = (num % 3) as usize;
            result[idx].insert(num);
        }

        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, true);
    }
}
