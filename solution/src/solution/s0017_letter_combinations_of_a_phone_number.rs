#![allow(dead_code)]

/// reference: https://leetcode.cn/problems/letter-combinations-of-a-phone-number/
pub struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return Vec::new();
        }
        let digits = digits.into_bytes();
        fn dfs(u: usize, path: &mut String, digits: &Vec<u8>, result: &mut Vec<String>) {
            if u == digits.len() {
                result.push(path.clone());
                return;
            }
            let alpha: Vec<u8> = match digits[u] {
                b'2' => "abc",
                b'3' => "def",
                b'4' => "ghi",
                b'5' => "jkl",
                b'6' => "mno",
                b'7' => "pqrs",
                b'8' => "tuv",
                b'9' => "wxyz",
                _ => unreachable!(),
            }
            .into();

            for a in alpha {
                path.push(a as char);
                dfs(u + 1, path, digits, result);
                path.pop();
            }
        }

        let mut result = Vec::new();
        dfs(0, &mut String::new(), &digits, &mut result);
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
            vec_into!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"] as Vec<String>,
            Solution::letter_combinations("23".into())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![] as Vec<String>,
            Solution::letter_combinations("".into())
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            vec_into!["a", "b", "c"] as Vec<String>,
            Solution::letter_combinations("2".into())
        );
    }
}
