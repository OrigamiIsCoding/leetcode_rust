#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/number-of-burgers-with-no-waste-of-ingredients/
pub struct Solution;

impl Solution {
    fn determinant(matrix: [[i32; 2]; 2]) -> i32 {
        matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0]
    }

    pub fn num_of_burgers(tomato_slices: i32, cheese_slices: i32) -> Vec<i32> {
        let d = Self::determinant([[4, 2], [1, 1]]);
        let jumbo = Self::determinant([[tomato_slices, 2], [cheese_slices, 1]]);
        let small = Self::determinant([[4, tomato_slices], [1, cheese_slices]]);
        if jumbo % d != 0 || small % d != 0 || jumbo < 0 || small < 0 {
            vec![]
        } else {
            vec![jumbo / d, small / d]
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use test_case::test_case;

    #[test_case(1, 2, vec![1,2])]
    #[test_case(17, 4, vec![])]
    #[test_case(4, 17, vec![])]
    #[test_case(0, 0, vec![0, 0])]
    #[test_case(2, 1, vec![0, 1])]
    fn test(x: i32, y: i32, r: Vec<i32>) {
        assert_eq!(r, Solution::num_of_burgers(x, y));
    }
}
