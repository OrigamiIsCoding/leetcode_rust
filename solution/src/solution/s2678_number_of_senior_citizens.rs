#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/number-of-senior-citizens/
pub struct Solution;

impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        details
            .into_iter()
            .filter_map(|v| v[11..13].parse::<i32>().ok().filter(|&v| v > 60))
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]

    use super::*;
    use crate::vec_into;

    #[test]
    fn test_1() {
        assert_eq!(
            2,
            Solution::count_seniors(vec_into![
                "7868190130M7522",
                "5303914400F9211",
                "9273338290F4010"
            ])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            0,
            Solution::count_seniors(vec_into!["1313579440F2036", "2921522980M5644"])
        );
    }
}
