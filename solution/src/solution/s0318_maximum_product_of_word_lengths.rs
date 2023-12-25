#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/maximum-product-of-word-lengths/
pub struct Solution;

impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let words: Vec<_> = words
            .into_iter()
            .map(|s| {
                (
                    s.len() as i32,
                    s.bytes().fold(0, |acc, b| acc | 1 << (b - b'a')),
                )
            })
            .collect();
        let mut ans = 0;
        for (i, &(len1, mask1)) in words.iter().enumerate() {
            for &(len2, mask2) in words.iter().skip(i + 1) {
                if (mask1 & mask2) == 0 {
                    ans = i32::max(ans, len1 * len2);
                }
            }
        }
        ans
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
            16,
            Solution::max_product(vec_into!["abcw", "baz", "foo", "bar", "xtfn", "abcdef"])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            4,
            Solution::max_product(vec_into!["a", "ab", "abc", "d", "cd", "bcd", "abcd"])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            0,
            Solution::max_product(vec_into!["a", "aa", "aaa", "aaaa"])
        );
    }
}
