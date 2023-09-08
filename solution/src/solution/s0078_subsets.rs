#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/subsets/
pub struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();

        fn dfs(u: usize, nums: &Vec<i32>, ans: &mut Vec<Vec<i32>>, path: &mut Vec<i32>) {
            match nums.get(u) {
                Some(&value) => {
                    dfs(u + 1, nums, ans, path);
                    path.push(value);
                    dfs(u + 1, nums, ans, path);
                    path.pop();
                }
                None => ans.push(path.clone()),
            }
        }

        dfs(0, &nums, &mut ans, &mut Vec::new());
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::mat;

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            mat![[], [1], [2], [1, 2], [3], [1, 3], [2, 3], [1, 2, 3]],
            Solution::subsets(vec![1, 2, 3])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(mat![[], [0]], Solution::subsets(vec![0]));
    }
}
