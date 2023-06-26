#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/incremental-memory-leak/
pub struct Solution;

impl Solution {
    pub fn mem_leak(mut memory1: i32, mut memory2: i32) -> Vec<i32> {
        let mut t = 1;
        loop {
            let m = if memory1 < memory2 {
                &mut memory2
            } else {
                &mut memory1
            };
            if *m < t {
                break vec![t, memory1, memory2];
            } else {
                *m -= t;
                t += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![3, 1, 0], Solution::mem_leak(2, 2));
    }

    #[test]
    fn test_2() {
        assert_eq!(vec![6, 0, 4], Solution::mem_leak(8, 11));
    }
}
