#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/peak-index-in-a-mountain-array/
pub struct Solution;

impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, arr.len() - 1);
        while l < r {
            let mid = (l + r) / 2;
            if arr[mid] > arr[mid + 1] {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        l as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::peak_index_in_mountain_array(vec![0, 1, 0]));
    }

    #[test]
    fn test_2() {
        assert_eq!(1, Solution::peak_index_in_mountain_array(vec![0, 2, 1, 0]));
    }

    #[test]
    fn test_3() {
        assert_eq!(1, Solution::peak_index_in_mountain_array(vec![0, 10, 5, 2]));
    }
}
