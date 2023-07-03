#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/plus-one/
pub struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        digits.reverse();
        let mut t = 1;
        for d in digits.iter_mut() {
            *d += t;
            t = *d / 10;
            *d %= 10;
            if t == 0 {
                break;
            }
        }
        if t != 0 {
            digits.push(t);
        }
        digits.reverse();
        digits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![1, 2, 4], Solution::plus_one(vec![1, 2, 3]));
    }

    #[test]
    fn test_2() {
        assert_eq!(vec![4, 3, 2, 2], Solution::plus_one(vec![4, 3, 2, 1]));
    }

    #[test]
    fn test_3() {
        assert_eq!(vec![1], Solution::plus_one(vec![0]));
    }
}
