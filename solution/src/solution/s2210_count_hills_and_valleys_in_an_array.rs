#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/count-hills-and-valleys-in-an-array/
pub struct Solution;

impl Solution {
    pub fn count_hill_valley(mut nums: Vec<i32>) -> i32 {
        nums.dedup();
        nums.iter()
            .zip(nums.iter().skip(1))
            .zip(nums.iter().skip(2))
            .filter(|((&left, &mid), &right)| {
                (left > mid && right > mid) || (left < mid && right < mid)
            })
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::count_hill_valley(vec![2, 4, 1, 1, 6, 5]));
    }

    #[test]
    fn test_2() {
        assert_eq!(0, Solution::count_hill_valley(vec![6, 6, 5, 5, 4, 1]));
    }
}
