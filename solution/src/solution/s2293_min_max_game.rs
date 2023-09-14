#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/min-max-game/
pub struct Solution;

impl Solution {
    pub fn min_max_game(nums: Vec<i32>) -> i32 {
        Self::min_max_game_impl(nums)
    }

    fn min_max_game_impl(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        Self::min_max_game_impl(
            (0..nums.len() / 2)
                .map(|i| {
                    if i % 2 == 0 {
                        i32::min(nums[i * 2], nums[i * 2 + 1])
                    } else {
                        i32::max(nums[i * 2], nums[i * 2 + 1])
                    }
                })
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::min_max_game(vec![1, 3, 5, 2, 4, 8, 2, 2]));
    }

    #[test]
    fn test_2() {
        assert_eq!(3, Solution::min_max_game(vec![3]));
    }
}
