#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/maximum-bags-with-full-capacity-of-rocks/
pub struct Solution;

impl Solution {
    pub fn maximum_bags(capacity: Vec<i32>, rocks: Vec<i32>, mut additional_rocks: i32) -> i32 {
        let mut remains: Vec<_> = capacity
            .into_iter()
            .zip(rocks)
            .map(|(c, r)| c - r)
            .collect();
        remains.sort();

        remains
            .into_iter()
            .take_while(|r| match additional_rocks.cmp(r) {
                std::cmp::Ordering::Less => false,
                _ => {
                    additional_rocks -= *r;
                    true
                }
            })
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            3,
            Solution::maximum_bags(vec![2, 3, 4, 5], vec![1, 2, 4, 4], 2)
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            3,
            Solution::maximum_bags(vec![10, 2, 2], vec![2, 2, 0], 100)
        );
    }
}
