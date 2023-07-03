#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/valid-parentheses/
pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let h: std::collections::HashMap<_, _> = vec![(b')', b'('), (b'}', b'{'), (b']', b'[')]
            .into_iter()
            .collect();
        let mut stk = Vec::new();

        for b in s.bytes() {
            match stk.last() {
                Some(&top) => match h.get(&b) {
                    Some(&r) if r == top => {
                        stk.pop();
                    }
                    _ => stk.push(b),
                },
                _ => stk.push(b),
            }
        }
        stk.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::is_valid("()".into()));
    }

    #[test]
    fn test_2() {
        assert_eq!(true, Solution::is_valid("()[]{}".into()));
    }

    #[test]
    fn test_3() {
        assert_eq!(false, Solution::is_valid("(]".into()));
    }

    #[test]
    fn test_4() {
        assert_eq!(true, Solution::is_valid("{[]}".into()));
    }
}
