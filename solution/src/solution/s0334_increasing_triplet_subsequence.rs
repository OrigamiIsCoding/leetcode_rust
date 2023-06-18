#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/increasing-triplet-subsequence/
pub struct Solution;

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        // longest increasing subsequence
        let mut lis = [0; 4];
        let mut len = 0;
        for num in nums {
            let (mut l, mut r) = (0, len);
            while l < r {
                let mid = (l + r + 1) / 2;
                if lis[mid] >= num {
                    r = mid - 1;
                } else {
                    l = mid;
                }
            }
            len = len.max(l + 1);
            lis[l + 1] = num;

            if len >= 3 {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::increasing_triplet(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn test_2() {
        assert_eq!(false, Solution::increasing_triplet(vec![5, 4, 3, 2, 1]));
    }

    #[test]
    fn test_3() {
        assert_eq!(true, Solution::increasing_triplet(vec![2, 1, 5, 0, 4, 6]));
    }

    #[test]
    fn test_4() {
        assert_eq!(false, Solution::increasing_triplet(vec![1; 200]))
    }
}
