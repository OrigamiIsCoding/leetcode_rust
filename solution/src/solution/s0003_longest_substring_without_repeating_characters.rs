#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/longest-substring-without-repeating-characters/
pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let s = s.into_bytes();
        let mut h = [false; 256];
        let (mut l, mut ans) = (0, 0);
        for (r, &b) in s.iter().enumerate() {
            while h[b as usize] {
                h[s[l] as usize] = false;
                l += 1;
            }
            h[s[r] as usize] = true;
            ans = i32::max(ans, (r - l + 1) as i32);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::length_of_longest_substring("abcabcbb".into()));
    }

    #[test]
    fn test_2() {
        assert_eq!(1, Solution::length_of_longest_substring("bbbbb".into()));
    }

    #[test]
    fn test_3() {
        assert_eq!(3, Solution::length_of_longest_substring("pwwkew".into()));
    }
}
