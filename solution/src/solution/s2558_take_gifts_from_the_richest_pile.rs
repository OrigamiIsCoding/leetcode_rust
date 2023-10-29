#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/take-gifts-from-the-richest-pile/
pub struct Solution;

impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        let mut heap = std::collections::BinaryHeap::from(gifts);
        for _ in 0..k {
            if let Some(count) = heap.pop() {
                heap.push((count as f64).sqrt().floor() as i32)
            }
        }
        heap.into_iter().map(|v| v as i64).sum()
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(29, Solution::pick_gifts(vec![25, 64, 9, 4, 100], 4));
    }

    #[test]
    fn test_2() {
        assert_eq!(4, Solution::pick_gifts(vec![1, 1, 1, 1], 4));
    }
}
