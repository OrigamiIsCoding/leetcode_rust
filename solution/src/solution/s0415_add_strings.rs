#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/add-strings/
pub struct Solution;

struct BigInteger {
    nums: Vec<u8>,
}

impl BigInteger {
    fn len(&self) -> usize {
        self.nums.len()
    }
}

impl std::ops::Index<usize> for BigInteger {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.nums[index]
    }
}

impl From<String> for BigInteger {
    fn from(value: String) -> Self {
        Self {
            nums: value.bytes().rev().map(|b| b - b'0').collect(),
        }
    }
}

impl From<BigInteger> for String {
    fn from(value: BigInteger) -> Self {
        value
            .nums
            .into_iter()
            .rev()
            .map(|num| (num + b'0') as char)
            .collect()
    }
}

impl std::ops::Add for BigInteger {
    type Output = BigInteger;

    fn add(self, rhs: Self) -> Self::Output {
        let mut t = 0;
        let mut nums = Vec::new();
        for i in 0..self.len().max(rhs.len()) {
            if i < self.len() {
                t += self[i];
            }
            if i < rhs.len() {
                t += rhs[i];
            }
            nums.push(t % 10);
            t /= 10;
        }

        if t > 0 {
            nums.push(t);
        }

        Self { nums }
    }
}

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let (num1, num2) = (BigInteger::from(num1), BigInteger::from(num2));
        (num1 + num2).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!("134", Solution::add_strings("11".into(), "123".into()));
    }

    #[test]
    fn test_2() {
        assert_eq!("533", Solution::add_strings("456".into(), "77".into()));
    }

    #[test]
    fn test_3() {
        assert_eq!("0", Solution::add_strings("0".into(), "0".into()));
    }
}
