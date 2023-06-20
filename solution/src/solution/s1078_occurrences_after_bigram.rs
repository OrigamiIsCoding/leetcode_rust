#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/occurrences-after-bigram/
pub struct Solution;

impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        text.split_whitespace()
            .collect::<Vec<_>>()
            .windows(3)
            .filter(|window| window[..2] == [&first, &second])
            .map(|window| window[2].to_string())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::vec_into;

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec_into!["girl", "student"] as Vec<String>,
            Solution::find_ocurrences(
                "alice is a good girl she is a good student".into(),
                "a".into(),
                "good".into()
            )
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec_into!["we", "rock"] as Vec<String>,
            Solution::find_ocurrences(
                "we will we will rock you".into(),
                "we".into(),
                "will".into()
            )
        );
    }
}
