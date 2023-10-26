#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/flip-string-to-monotone-increasing/
pub struct Solution;

impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        // f[i][j] 表示第 i 个以 j 结尾的最小花费
        let mut f = vec![vec![0; 2]; 2];
        for (idx, b) in s.bytes().enumerate() {
            f[(idx + 1) & 1][0] = f[idx & 1][0] + if b == b'0' { 0 } else { 1 };
            f[(idx + 1) & 1][1] =
                i32::min(f[idx & 1][0], f[idx & 1][1]) + if b == b'1' { 0 } else { 1 };
        }
        f[s.len() & 1].iter().copied().min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::min_flips_mono_incr("00110".into()));
    }

    #[test]
    fn test_2() {
        assert_eq!(2, Solution::min_flips_mono_incr("010110".into()));
    }

    #[test]
    fn test_3() {
        assert_eq!(2, Solution::min_flips_mono_incr("00011000".into()));
    }
}
