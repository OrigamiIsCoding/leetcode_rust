#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/minimum-time-to-repair-cars/
pub struct Solution;
use std::cmp::Reverse;

impl Solution {
    pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
        let mut heap = std::collections::BinaryHeap::from(
            ranks
                .into_iter()
                .map(|val| Reverse((val as i64, val as i64, 1_i64)))
                .collect::<Vec<_>>(),
        );
        for _ in 0..cars {
            if let Some(mut top) = heap.peek_mut() {
                let (_, r, count) = top.0;
                top.0 = (r * (count + 1).pow(2), r, count + 1)
            }
        }
        heap.into_iter()
            .map(|Reverse((_, r, count))| r * (count - 1).pow(2))
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(16, Solution::repair_cars(vec![4, 2, 3, 1], 10));
    }

    #[test]
    fn test_2() {
        assert_eq!(16, Solution::repair_cars(vec![5, 1, 8], 6));
    }
}
