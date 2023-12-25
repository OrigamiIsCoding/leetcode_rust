#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/house-robber-ii/
pub struct Solution;

impl Solution {
    pub fn rob(_nums: Vec<i32>) -> i32 {
        fn solve(iter: &mut dyn Iterator<Item = &i32>) -> i32 {
            let (mut a, mut b) = (0, 0);
            iter.enumerate().for_each(|(i, &v)| {
                if i % 2 == 0 {
                    a = i32::max(a + v, b);
                } else {
                    b = i32::max(b + v, a);
                }
            });
            i32::max(a, b)
        }
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::rob(vec![2, 3, 2]));
    }

    #[test]
    fn test_2() {
        assert_eq!(4, Solution::rob(vec![1, 2, 3, 1]))
    }

    #[test]
    fn test_3() {
        assert_eq!(3, Solution::rob(vec![1, 2, 3]))
    }
}
