#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/find-all-numbers-disappeared-in-an-array/
pub struct Solution;

impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        for i in 0..nums.len() {
            let j = (nums[i] - 1) as usize % n;
            nums[j] += n as i32;
        }
        nums.into_iter()
            .enumerate()
            .filter_map(|(index, num)| {
                if num as usize <= n {
                    Some(index as i32 + 1)
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![5, 6],
            Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(vec![2], Solution::find_disappeared_numbers(vec![1, 1]));
    }
}
