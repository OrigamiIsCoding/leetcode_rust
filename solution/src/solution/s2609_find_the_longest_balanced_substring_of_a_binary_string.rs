#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/find-the-longest-balanced-substring-of-a-binary-string/
pub struct Solution;

impl Solution {
    pub fn find_the_longest_balanced_substring(s: String) -> i32 {
        let s: Vec<_> = s.bytes().collect();
        let mut len = s.len() / 2 * 2;
        while len > 0 {
            if s.windows(len).any(|slice| {
                let count0 = slice.iter().take(len / 2).filter(|&&b| b == b'0').count();
                let count1 = slice.iter().skip(len / 2).filter(|&&b| b == b'1').count();
                count0 == count1 && count0 + count1 == len
            }) {
                return len as i32;
            }
            len -= 2;
        }
        0
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            6,
            Solution::find_the_longest_balanced_substring("01000111".into())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            4,
            Solution::find_the_longest_balanced_substring("00111".into())
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            0,
            Solution::find_the_longest_balanced_substring("000".into())
        );
    }
}
