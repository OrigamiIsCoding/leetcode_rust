#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/longest-common-prefix/
pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let strs: Vec<_> = strs.into_iter().map(|str| str.into_bytes()).collect();
        let min_len = strs.iter().map(|str| str.len()).min().unwrap();
        let mut result = String::new();

        for i in 0..min_len {
            let b = strs[0][i];
            if strs.iter().all(|str| str[i] == b) {
                result.push(b as char);
            } else {
                break;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::vec_into;

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "fl".to_string(),
            Solution::longest_common_prefix(vec_into!["flower", "flow", "flight"])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "".to_string(),
            Solution::longest_common_prefix(vec_into!["dog", "racecar", "car"])
        );
    }
}
