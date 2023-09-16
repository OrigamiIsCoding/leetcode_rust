#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/number-of-ways-to-buy-pens-and-pencils/
pub struct Solution;

impl Solution {
    pub fn ways_to_buy_pens_pencils(total: i32, cost1: i32, cost2: i32) -> i64 {
        let mut f = vec![0_i64; (total + 1) as usize];
        f[0] = 1;
        for cost in [cost1, cost2] {
            for j in cost..=total {
                f[j as usize] += f[(j - cost) as usize];
            }
        }
        f.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(9, Solution::ways_to_buy_pens_pencils(20, 10, 5));
    }

    #[test]
    fn test_2() {
        assert_eq!(1, Solution::ways_to_buy_pens_pencils(5, 10, 10));
    }
}
