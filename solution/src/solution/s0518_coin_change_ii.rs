#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/coin-change-ii/
pub struct Solution;

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let mut f = vec![0; amount as usize + 1];

        f[0] = 1;
        for coin in coins {
            for v in coin..=amount {
                f[v as usize] += f[(v - coin) as usize];
            }
        }

        f[amount as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::change(5, vec![1, 2, 5]));
    }

    #[test]
    fn test_2() {
        assert_eq!(0, Solution::change(3, vec![2]));
    }

    #[test]
    fn test_3() {
        assert_eq!(1, Solution::change(10, vec![10]));
    }
}
