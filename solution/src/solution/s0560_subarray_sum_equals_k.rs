#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/subarray-sum-equals-k/
pub struct Solution;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut h = std::collections::HashMap::new();
        h.insert(0, 1);

        let (mut result, mut acc) = (0, 0);

        for num in nums {
            // acc - s[l] = k
            // s[l] = acc - k
            acc += num;
            result += h.get(&(acc - k)).unwrap_or(&0);
            *h.entry(acc).or_insert(0) += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::subarray_sum(vec![1, 1, 1], 2));
    }

    #[test]
    fn test_2() {
        assert_eq!(2, Solution::subarray_sum(vec![1, 2, 3], 3));
    }
}
