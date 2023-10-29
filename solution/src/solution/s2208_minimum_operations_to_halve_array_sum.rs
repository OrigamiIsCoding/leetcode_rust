#![allow(dead_code)]
#![allow(clippy::derive_ord_xor_partial_ord)]
/// reference: https://leetcode.cn/problems/minimum-operations-to-halve-array-sum/
pub struct Solution;

use std::cmp::Ordering;

#[derive(PartialOrd, PartialEq, Copy, Clone)]
struct F64(f64);

impl Eq for F64 {}

impl Ord for F64 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Solution {
    pub fn halve_array(nums: Vec<i32>) -> i32 {
        let mut heap = std::collections::BinaryHeap::new();
        let (mut sum, mut current, mut ans) = (0.0, 0.0, 0);
        nums.into_iter().for_each(|x| {
            sum += x as f64;
            heap.push(F64(x as f64));
        });
        while current < sum / 2.0 {
            if let Some(F64(value)) = heap.pop() {
                let value = value / 2_f64;
                current += value;
                ans += 1;
                heap.push(F64(value));
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
        assert_eq!(3, Solution::halve_array(vec![5, 19, 8, 1]));
    }

    #[test]
    fn test_2() {
        assert_eq!(3, Solution::halve_array(vec![3, 8, 20]));
    }
}
