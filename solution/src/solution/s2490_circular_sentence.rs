#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/circular-sentence/
pub struct Solution;

impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let mut splitted: Vec<_> = sentence.split_whitespace().map(|s| s.bytes().collect::<Vec<_>>()).collect();
        splitted.push(splitted[0].clone());
        splitted
            .iter()
            .zip(splitted.iter().skip(1))
            .all(|(left, right)| left.last() == right.first())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            true,
            Solution::is_circular_sentence("leetcode exercises sound delightful".into())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(true, Solution::is_circular_sentence("eetcode".into()));
    }

    #[test]
    fn test_3() {
        assert_eq!(
            false,
            Solution::is_circular_sentence("Leetcode is cool".into())
        );
    }
}
