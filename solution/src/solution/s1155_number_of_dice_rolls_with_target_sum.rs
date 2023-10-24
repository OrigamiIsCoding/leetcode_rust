#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/number-of-dice-rolls-with-target-sum/
pub struct Solution;

impl Solution {
    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        const MOD: i32 = 1_000_000_000 + 7;
        let (n, k, target) = (n as usize, k as usize, target as usize);
        let mut dp = vec![vec![0; target + 1]; 2];
        dp[0][0] = 1;
        for i in 1..=n {
            dp[i & 1].fill(0);
            for j in 1..=k {
                for t in j..=target {
                    dp[i & 1][t] = (dp[i & 1][t] + dp[i & 1 ^ 1][t - j]) % MOD;
                }
            }
        }
        dp[n & 1][target]
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::num_rolls_to_target(1, 6, 3));
    }
    #[test]
    fn test_2() {
        assert_eq!(6, Solution::num_rolls_to_target(2, 6, 7));
    }
    #[test]
    fn test_3() {
        assert_eq!(222616187, Solution::num_rolls_to_target(30, 30, 500));
    }
}
