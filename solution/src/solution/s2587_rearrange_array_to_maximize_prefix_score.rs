#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/rearrange-array-to-maximize-prefix-score/
pub struct Solution;

impl Solution {
    pub fn max_score(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        nums.into_iter()
            .rev()
            .fold((0, 0), |(mut ans, mut prefix_sum), v| {
                prefix_sum += v as i64;
                if prefix_sum > 0 {
                    ans += 1;
                }
                (ans, prefix_sum)
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(6, Solution::max_score(vec![2, -1, 0, 1, -3, 3, -3]));
    }

    #[test]
    fn test_2() {
        assert_eq!(0, Solution::max_score(vec![-2, -3, 0]));
    }
}
