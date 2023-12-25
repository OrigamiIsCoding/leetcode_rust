#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/sum-multiples/
pub struct Solution;

impl Solution {
    pub fn sum_of_multiples(n: i32) -> i32 {
        (1..=n)
            .filter(|&i| i % 3 == 0 || i % 5 == 0 || i % 7 == 0)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(21, Solution::sum_of_multiples(7));
    }
    #[test]
    fn test_2() {
        assert_eq!(40, Solution::sum_of_multiples(10));
    }
    #[test]
    fn test_3() {
        assert_eq!(30, Solution::sum_of_multiples(9));
    }
}
