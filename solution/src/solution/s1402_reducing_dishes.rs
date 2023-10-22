#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/reducing-dishes/
pub struct Solution;

impl Solution {
    pub fn max_satisfaction(mut satisfaction: Vec<i32>) -> i32 {
        satisfaction.sort();
        let (mut suffix_sum, mut current) = (0, 0);
        let mut result = 0;
        for (idx, &s) in satisfaction.iter().enumerate() {
            suffix_sum += s;
            current += (idx + 1) as i32 * s;
        }
        satisfaction
            .into_iter()
            .take_while(|&s| s <= 0)
            .for_each(|s| {
                result = i32::max(result, current);
                current -= suffix_sum;
                suffix_sum -= s;
            });
        i32::max(result, current)
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(14, Solution::max_satisfaction(vec![-1, -8, 0, 5, -9]));
    }

    #[test]
    fn test_2() {
        assert_eq!(20, Solution::max_satisfaction(vec![4, 3, 2]));
    }

    #[test]
    fn test_3() {
        assert_eq!(0, Solution::max_satisfaction(vec![-1, -4, -5]));
    }
}
