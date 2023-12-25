#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/how-many-numbers-are-smaller-than-the-current-number/
pub struct Solution;

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut counter = nums.iter().fold(vec![0; 100], |mut acc, &v| {
            if let Some(v) = acc.get_mut(v as usize) {
                *v += 1;
            }
            acc
        });
        for i in 1..100 {
            counter[i] += counter[i - 1]
        }
        nums.into_iter()
            .map(|v| *counter.get((v - 1) as usize).unwrap_or(&0))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![4, 0, 1, 1, 3],
            Solution::smaller_numbers_than_current(vec![8, 1, 2, 2, 3])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![2, 1, 0, 3],
            Solution::smaller_numbers_than_current(vec![6, 5, 4, 8])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            vec![0, 0, 0, 0],
            Solution::smaller_numbers_than_current(vec![7, 7, 7, 7])
        );
    }
}
