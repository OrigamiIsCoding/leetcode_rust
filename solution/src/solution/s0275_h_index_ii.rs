#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/h-index-ii/
pub struct Solution;

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        (0..citations.len())
            .collect::<Vec<_>>()
            .binary_search_by(|&ans| (ans as i32).cmp(&citations[citations.len() - ans - 1]))
            .unwrap_or_else(|err| err) as i32
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::h_index(vec![0, 1, 3, 5, 6]));
    }

    #[test]
    fn test_2() {
        assert_eq!(2, Solution::h_index(vec![1, 2, 100]));
    }
}
