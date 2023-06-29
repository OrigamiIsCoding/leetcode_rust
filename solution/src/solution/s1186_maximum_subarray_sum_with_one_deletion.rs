#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/maximum-subarray-sum-with-one-deletion/
pub struct Solution;

impl Solution {
    pub fn maximum_sum(arr: Vec<i32>) -> i32 {
        // f[i][0] 不删除一个的子数组最大和
        // f[i][1] 删除一次的子数组最大和
        let mut f = [[0; 2]; 2];
        let mut result = arr[0];
        f[0][0] = i32::MIN;
        for (i, v) in arr.into_iter().enumerate() {
            f[(i + 1) & 1][0] = i32::max(0, f[i & 1][0]) + v;
            f[(i + 1) & 1][1] = i32::max(f[i & 1][0], f[i & 1][1] + v);
            result = i32::max(result, i32::max(f[(i + 1) & 1][0], f[(i + 1) & 1][1]));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::maximum_sum(vec![1, -2, 0, 3]));
    }

    #[test]
    fn test_2() {
        assert_eq!(3, Solution::maximum_sum(vec![1, -2, -2, 3]));
    }

    #[test]
    fn test_3() {
        assert_eq!(-1, Solution::maximum_sum(vec![-1, -1, -1, -1]));
    }

    #[test]
    fn test_4() {
        assert_eq!(3, Solution::maximum_sum(vec![2, 1, -2, -5, -2]));
    }
}
