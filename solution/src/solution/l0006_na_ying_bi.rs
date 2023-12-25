#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/na-ying-bi/
pub struct Solution;

impl Solution {
    pub fn min_count(coins: Vec<i32>) -> i32 {
        coins.into_iter().map(|coin| coin / 2 + coin % 2).sum()
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::min_count(vec![4, 2, 1]));
    }

    #[test]
    fn test_2() {
        assert_eq!(8, Solution::min_count(vec![2, 3, 10]));
    }
}
