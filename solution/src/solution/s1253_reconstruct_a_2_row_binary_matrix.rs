#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/reconstruct-a-2-row-binary-matrix/
pub struct Solution;

impl Solution {
    pub fn reconstruct_matrix(mut upper: i32, mut lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![vec![0; colsum.len()], vec![0; colsum.len()]];

        for (i, &col) in colsum.iter().enumerate() {
            if col == 2 {
                result[0][i] = 1;
                result[1][i] = 1;
                upper -= 1;
                lower -= 1;
            } else if col == 1 {
                if upper > lower {
                    upper -= 1;
                    result[0][i] = 1;
                } else {
                    lower -= 1;
                    result[1][i] = 1;
                }
            }
        }

        if upper != 0 || lower != 0 {
            Vec::new()
        } else {
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mat;

    #[test]
    fn test_1() {
        let actual = Solution::reconstruct_matrix(2, 1, vec![1, 1, 1]);
        assert!([
            mat![[1, 1, 0], [0, 0, 1]],
            mat![[1, 0, 1], [0, 1, 0]],
            mat![[0, 1, 1], [1, 0, 0]]
        ]
        .into_iter()
        .any(|excepted| excepted == actual))
    }

    #[test]
    fn test_2() {
        assert_eq!(
            mat![] as Vec<Vec<i32>>,
            Solution::reconstruct_matrix(2, 3, vec![2, 2, 1, 1])
        );
    }

    // #[test]
    // fn test_3() {
    //     assert_eq!(
    //         mat![
    //             [1, 1, 1, 0, 1, 0, 0, 1, 0, 0],
    //             [1, 0, 1, 0, 0, 0, 1, 1, 0, 1]
    //         ],
    //         Solution::reconstruct_matrix(5, 5, vec![2, 1, 2, 0, 1, 0, 1, 2, 0, 1])
    //     );
    // }
}
