#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/find-the-punishment-number-of-an-integer/
pub struct Solution;

impl Solution {
    pub fn punishment_number(n: i32) -> i32 {
        fn check(mut n: i32, target: i32) -> bool {
            fn dfs(nums: &Vec<i32>, u: usize, prev: i32, sum: i32, target: i32) -> bool {
                if sum + prev > target {
                    return false;
                }
                if nums.len() == u {
                    return sum + prev == target;
                }
                if dfs(nums, u + 1, prev * 10 + nums[u], sum, target) {
                    true
                } else {
                    dfs(nums, u + 1, 0, prev * 10 + nums[u] + sum, target)
                }
            }
            let mut nums = Vec::new();
            while n != 0 {
                nums.push(n % 10);
                n /= 10;
            }
            nums.reverse();
            dfs(&nums, 0, 0, 0, target)
        }
        (1..=n)
            .filter_map(|t| {
                let n = t * t;
                if check(n, t) {
                    Some(n)
                } else {
                    None
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(182, Solution::punishment_number(10));
    }
    #[test]
    fn test_2() {
        assert_eq!(1478, Solution::punishment_number(37));
    }
}
