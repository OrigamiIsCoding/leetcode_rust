#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/count-the-number-of-vowel-strings-in-range/
pub struct Solution;

impl Solution {
    pub fn vowel_strings(words: Vec<String>, left: i32, right: i32) -> i32 {
        words
            .into_iter()
            .skip(left as usize)
            .take((right - left + 1) as usize)
            .filter(|word| {
                word.starts_with(&['a', 'e', 'i', 'o', 'u'])
                    && word.ends_with(&['a', 'e', 'i', 'o', 'u'])
            })
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
            Solution::vowel_strings(vec_into!["are", "amy", "u"], 0, 2)
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            3,
            Solution::vowel_strings(vec_into!["hey", "aeo", "mu", "ooo", "artro"], 1, 4)
        );
    }
}
