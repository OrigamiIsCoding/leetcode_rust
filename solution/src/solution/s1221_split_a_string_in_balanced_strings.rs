#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/split-a-string-in-balanced-strings/
pub struct Solution;

impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        s.bytes()
            .fold((0, 0, 0), |acc, b| {
                let (lc, rc, result) = acc;
                match b {
                    b'R' if lc == rc + 1 => (0, 0, result + 1),
                    b'L' if rc == lc + 1 => (0, 0, result + 1),
                    b'R' => (lc, rc + 1, result),
                    b'L' => (lc + 1, rc, result),
                    _ => (-1, -1, -1),
                }
            })
            .2
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::balanced_string_split("RLRRLLRLRL".into()));
    }

    #[test]
    fn test_2() {
        assert_eq!(2, Solution::balanced_string_split("RLRRRLLRLL".into()));
    }

    #[test]
    fn test_3() {
        assert_eq!(1, Solution::balanced_string_split("LLLLRRRR".into()));
    }
}
