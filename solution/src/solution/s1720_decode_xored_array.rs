#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/decode-xored-array/
pub struct Solution;

impl Solution {
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let mut result = vec![first];
        encoded.into_iter().for_each(|v| {
            result.push(v ^ result.last().unwrap());
        });
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![1, 0, 2, 1], Solution::decode(vec![1, 2, 3], 1));
    }

    #[test]
    fn test_2() {
        assert_eq!(vec![4, 2, 0, 7, 4], Solution::decode(vec![6, 2, 7, 3], 4));
    }
}
