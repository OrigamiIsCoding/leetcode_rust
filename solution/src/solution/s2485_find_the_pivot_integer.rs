#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/find-the-pivot-integer/
pub struct Solution;

impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        // (1 + x) * x / 2 = (x + n) * (n - x + 1) / 2
        // (1 + x) * x = (x + n) * (n - x + 1)
        // x + x^2 = nx + n^2 - nx + n + x - x^2
        // 2 * x^2 = n^2 + n
        // x = sqrt((n^2 + n) / 2)
        let r = n * (n + 1) / 2;
        let t = f64::sqrt(r as f64) as i32;
        if t * t == r {
            t
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(6, Solution::pivot_integer(8));
    }

    #[test]
    fn test_2() {
        assert_eq!(1, Solution::pivot_integer(1));
    }

    #[test]
    fn test_3() {
        assert_eq!(-1, Solution::pivot_integer(4));
    }
}
