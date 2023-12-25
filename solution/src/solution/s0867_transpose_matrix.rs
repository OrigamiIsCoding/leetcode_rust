#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/transpose-matrix/
pub struct Solution;

impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (n, m) = (matrix.len(), matrix[0].len());
        let mut result = vec![vec![0; n]; m];
        for i in 0..n {
            for j in 0..m {
                result[j][i] = matrix[i][j];
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]

    use super::*;
    use crate::mat;

    #[test]
    fn test_1() {
        assert_eq!(
            mat![[1, 4, 7], [2, 5, 8], [3, 6, 9]],
            Solution::transpose(mat![[1, 2, 3], [4, 5, 6], [7, 8, 9]])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            mat![[1, 4, 7], [2, 5, 8], [3, 6, 9]],
            Solution::transpose(mat![[1, 2, 3], [4, 5, 6], [7, 8, 9]])
        );
    }
}
