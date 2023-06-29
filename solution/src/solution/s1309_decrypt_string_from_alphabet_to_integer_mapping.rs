#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/decrypt-string-from-alphabet-to-integer-mapping/
pub struct Solution;

impl Solution {
    pub fn freq_alphabets(s: String) -> String {
        let mut iter = s.into_bytes().into_iter().rev();

        let mut s = Vec::new();
        while let Some(b) = iter.next() {
            if b == b'#' {
                let b = iter.next().unwrap() - b'0';
                let a = iter.next().unwrap() - b'0';
                s.push((a * 10 + b - 1) + b'a');
            } else {
                s.push((b - b'1') + b'a');
            }
        }

        s.reverse();
        unsafe { String::from_utf8_unchecked(s) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "jkab".to_string(),
            Solution::freq_alphabets("10#11#12".into())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!("acz".to_string(), Solution::freq_alphabets("1326#".into()));
    }
}
