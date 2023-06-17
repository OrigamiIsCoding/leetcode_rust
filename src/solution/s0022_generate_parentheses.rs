#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/generate-parentheses/
pub struct Solution;

impl Solution {
    fn dfs(
        n: usize,
        left_count: usize,
        right_count: usize,
        path: &mut String,
        result: &mut Vec<String>,
    ) {
        if left_count + right_count == n * 2 {
            result.push(path.clone());
            return;
        }

        if left_count < n {
            path.push('(');
            Self::dfs(n, left_count + 1, right_count, path, result);
            path.pop();
        }
        if right_count < left_count {
            path.push(')');
            Self::dfs(n, left_count, right_count + 1, path, result);
            path.pop();
        }
    }

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = Vec::new();
        Self::dfs(n as usize, 0, 0, &mut String::new(), &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::vec_into;

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec_into!["((()))", "(()())", "(())()", "()(())", "()()()"] as Vec<String>,
            Solution::generate_parenthesis(3)
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec_into!["()"] as Vec<String>,
            Solution::generate_parenthesis(1)
        );
    }
}
