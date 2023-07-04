#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/sum-in-a-matrix/
pub struct Solution;

impl Solution {
    pub fn matrix_sum(mut nums: Vec<Vec<i32>>) -> i32 {
        nums.iter_mut().for_each(|r| r.sort());

        (0..nums[0].len())
            .map(|col| {
                nums.iter()
                    .map(|row| row.iter().nth_back(col).unwrap())
                    .max()
                    .unwrap()
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::mat;

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            15,
            Solution::matrix_sum(mat![[7, 2, 1], [6, 4, 2], [6, 5, 3], [3, 2, 1]])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(1, Solution::matrix_sum(mat![[1]]));
    }
}
