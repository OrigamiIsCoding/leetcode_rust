#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/count-total-number-of-colored-cells/
pub struct Solution;

impl Solution {
    pub fn colored_cells(n: i32) -> i64 {
        // (0 + 4n-4) * n / 2 + 1
        4 * (n - 1) as i64 * n as i64 / 2 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::colored_cells(1));
    }

    #[test]
    fn test_2() {
        assert_eq!(5, Solution::colored_cells(2));
    }

    #[test]
    fn test_3() {
        assert_eq!(13, Solution::colored_cells(3));
    }
}
