#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/triangle/
pub struct Solution;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let n = triangle.len();
        let mut f = vec![vec![0; triangle.len()]; 2];
        for (i, row) in triangle.into_iter().enumerate() {
            for (j, val) in row.into_iter().enumerate() {
                if j == 0 {
                    f[(i + 1) & 1][j] = f[i & 1][j] + val;
                } else if i == j {
                    f[(i + 1) & 1][j] = f[i & 1][j - 1] + val;
                } else {
                    f[(i + 1) & 1][j] = i32::min(f[i & 1][j], f[i & 1][j - 1]) + val;
                }
            }
        }
        *f[n & 1].iter().min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::mat;

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            11,
            Solution::minimum_total(mat![[2], [3, 4], [6, 5, 7], [4, 1, 8, 3]])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(-10, Solution::minimum_total(mat![[-10]]));
    }
}
