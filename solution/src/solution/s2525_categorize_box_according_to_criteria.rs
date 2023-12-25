#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/categorize-box-according-to-criteria/
pub struct Solution;

impl Solution {
    pub fn categorize_box(length: i32, width: i32, height: i32, mass: i32) -> String {
        let bulky = [length, width, height].into_iter().any(|v| v >= 10_000)
            || (length as i64) * (width as i64) * (height as i64) >= 1_000_000_000;
        let heavy = mass >= 100;
        match (bulky, heavy) {
            (true, true) => "Both",
            (false, false) => "Neither",
            (true, false) => "Bulky",
            (false, true) => "Heavy",
        }
        .into()
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "Heavy".to_string(),
            Solution::categorize_box(1000, 35, 700, 300)
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "Neither".to_string(),
            Solution::categorize_box(200, 50, 800, 50)
        );
    }
}
