#![allow(dead_code)]

/// reference: https://leetcode.cn/problems/h-index/
pub struct Solution;

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        (0..=citations.len())
            .collect::<Vec<_>>()
            .binary_search_by(|&h| {
                citations
                    .iter()
                    .filter(|&&v| v as usize >= h)
                    .count()
                    .cmp(&h)
                    .reverse()
            })
            .map_or_else(|err| err - 1, |ok| ok) as i32
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(3, Solution::h_index(vec![3, 0, 6, 1, 5]));
    }

    #[test]
    fn test_2() {
        assert_eq!(1, Solution::h_index(vec![1, 3, 1]));
    }
}
