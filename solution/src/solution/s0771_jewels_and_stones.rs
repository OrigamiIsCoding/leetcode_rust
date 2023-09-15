#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/jewels-and-stones/
pub struct Solution;

impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let h: std::collections::HashSet<_> = jewels.into_bytes().into_iter().collect();
        stones
            .into_bytes()
            .into_iter()
            .filter(|b| h.contains(b))
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            3,
            Solution::num_jewels_in_stones("aA".into(), "aAAbbbb".into())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(0, Solution::num_jewels_in_stones("z".into(), "ZZ".into()));
    }
}
