#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/WHnhjV/
pub struct Solution;

impl Solution {
    pub fn give_gem(mut gem: Vec<i32>, operations: Vec<Vec<i32>>) -> i32 {
        for operation in operations {
            let [x, y] = operation[..] else { panic!() };
            let d = gem[x as usize] / 2;
            gem[x as usize] -= d;
            gem[y as usize] += d;
        }
        gem.iter().max().unwrap() - gem.iter().min().unwrap()
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
            2,
            Solution::give_gem(vec![3, 1, 2], mat![[0, 2], [2, 1], [2, 0]])
        )
    }

    #[test]
    fn test_2() {
        assert_eq!(
            75,
            Solution::give_gem(vec![100, 0, 50, 100], mat![[0, 2], [0, 1], [3, 0], [3, 0]])
        )
    }

    #[test]
    fn test_3() {
        assert_eq!(
            0,
            Solution::give_gem(vec![0; 4], mat![[1, 2], [3, 1], [1, 2]])
        )
    }

    #[test]
    fn test_4() {
        assert_eq!(
            4,
            Solution::give_gem(
                vec![0, 2, 5, 4],
                mat![
                    [3, 2],
                    [3, 2],
                    [1, 3],
                    [0, 2],
                    [3, 0],
                    [3, 1],
                    [0, 3],
                    [2, 1],
                    [3, 0]
                ]
            )
        )
    }
}
