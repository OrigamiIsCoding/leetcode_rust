#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/alternating-digit-sum/
pub struct Solution;

impl Solution {
    pub fn alternate_digit_sum(n: i32) -> i32 {
        n.to_string()
            .bytes()
            .enumerate()
            .map(|(idx, b)| {
                if idx % 2 == 0 {
                    (b - b'0') as i32
                } else {
                    -((b - b'0') as i32)
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::alternate_digit_sum(521));
    }

    #[test]
    fn test_2() {
        assert_eq!(1, Solution::alternate_digit_sum(111));
    }

    #[test]
    fn test_3() {
        assert_eq!(0, Solution::alternate_digit_sum(886996));
    }
}
