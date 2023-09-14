#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/diagonal-traverse/
pub struct Solution;

impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let (mut x, mut y) = (0, 0);
        let (n, m) = (mat.len(), mat.first().unwrap().len());
        let mut flag = false;
        loop {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, true);
    }
}
