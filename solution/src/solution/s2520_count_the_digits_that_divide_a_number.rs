#![allow(dead_code)]


/// reference: https://leetcode.cn/problems/count-the-digits-that-divide-a-number/
pub struct Solution;

impl Solution {
    pub fn count_digits(num: i32) -> i32 {
        let mut n = num;
        let mut count = 0;
        while n != 0 {
            let last = n % 10;
            if num % last == 0 {
                count += 1;
            }
            n /= 10;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::count_digits(7));
    }

    #[test]
    fn test_2() {
        assert_eq!(2, Solution::count_digits(121));
    }
}
