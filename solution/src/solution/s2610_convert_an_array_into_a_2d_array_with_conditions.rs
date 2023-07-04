#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/convert-an-array-into-a-2d-array-with-conditions/
pub struct Solution;

impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut h = std::collections::HashMap::new();
        nums.into_iter()
            .for_each(|num| *h.entry(num).or_insert(0) += 1);

        let mut result = Vec::new();
        while !h.is_empty() {
            let mut row = Vec::new();
            h.retain(|&k, v| {
                *v -= 1;
                row.push(k);
                *v != 0
            });
            result.push(row);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::mat;

    use super::*;

    fn check(mut actual: Vec<Vec<i32>>, mut excepted: Vec<Vec<i32>>) -> bool {
        actual.iter_mut().for_each(|row| row.sort());
        excepted.iter_mut().for_each(|row| row.sort());
        actual == excepted
    }

    #[test]
    fn test_1() {
        assert!(check(
            mat![[1, 3, 4, 2], [1, 3], [1]],
            Solution::find_matrix(vec![1, 3, 4, 1, 2, 3, 1])
        ));
    }

    #[test]
    fn test_2() {
        assert!(check(
            mat![[4, 3, 2, 1]],
            Solution::find_matrix(vec![1, 2, 3, 4])
        ));
    }
}
