#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/fHi6rV/
pub struct Solution;

impl Solution {
    pub fn flip_chess(_chessboard: Vec<String>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::vec_into;

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            3,
            Solution::flip_chess(vec_into!["....X.", "....X.", "XOOO..", "......", "......"])
        );
    }
}
