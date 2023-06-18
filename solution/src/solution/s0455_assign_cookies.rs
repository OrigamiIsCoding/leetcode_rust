#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/assign-cookies/
pub struct Solution;

impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort();
        s.sort();
        let (mut gi, mut si) = (0, 0);
        while gi < g.len() && si < s.len() {
            if g[gi] <= s[si] {
                gi += 1;
                si += 1;
            } else {
                si += 1;
            }
        }
        gi as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            1,
            Solution::find_content_children(vec![1, 2, 3], vec![1, 1])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            2,
            Solution::find_content_children(vec![1, 2], vec![1, 2, 3])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            2,
            Solution::find_content_children(vec![10, 9, 8, 7], vec![5, 6, 7, 8])
        );
    }
}
