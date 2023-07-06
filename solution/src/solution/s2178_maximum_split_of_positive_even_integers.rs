#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/maximum-split-of-positive-even-integers/
pub struct Solution;

impl Solution {
    pub fn maximum_even_split(mut final_sum: i64) -> Vec<i64> {
        if final_sum % 2 != 0 {
            return Vec::new();
        }
        let mut base = 2;
        let mut result = Vec::new();
        while final_sum >= base {
            final_sum -= base;
            result.push(base);
            base += 2;
        }
        *result.last_mut().unwrap() += final_sum;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![2, 4, 6], Solution::maximum_even_split(12));
    }

    #[test]
    fn test_2() {
        assert_eq!(vec![] as Vec<i64>, Solution::maximum_even_split(7));
    }

    #[test]
    fn test_3() {
        assert_eq!(vec![2, 4, 6, 16], Solution::maximum_even_split(28));
    }
}
