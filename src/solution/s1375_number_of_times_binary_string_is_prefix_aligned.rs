#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/number-of-times-binary-string-is-prefix-aligned/
pub struct Solution;

use std::ops::AddAssign;
struct FenwickTree<T>
where
    T: AddAssign + Copy + Default,
{
    values: Vec<T>,
    size: usize,
}

impl<T> FenwickTree<T>
where
    T: AddAssign + Copy + Default,
{
    #[inline]
    fn lowbit(x: usize) -> usize {
        x & x.wrapping_neg()
    }

    fn new(size: usize) -> Self {
        Self {
            values: vec![Default::default(); size + 1],
            size,
        }
    }

    fn add(&mut self, mut index: usize, d: T) {
        while index <= self.size {
            self.values[index] += d;
            index += Self::lowbit(index);
        }
    }

    fn query(&self, mut right: usize) -> T {
        let mut sum = Default::default();
        while right > 0 {
            sum += self.values[right];
            right -= Self::lowbit(right);
        }
        sum
    }
}

impl Solution {
    pub fn num_times_all_blue(flips: Vec<i32>) -> i32 {
        let mut ft = FenwickTree::new(flips.len());
        let mut ans = 0;
        for flip in flips {
            ft.add(flip as usize, 1);
            let count = ft.query(ft.size);
            if count == ft.query(count) {
                ans += 1;
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
        assert_eq!(2, Solution::num_times_all_blue(vec![3, 2, 4, 1, 5]));
    }

    #[test]
    fn test_2() {
        assert_eq!(1, Solution::num_times_all_blue(vec![4, 1, 2, 3]));
    }
}
