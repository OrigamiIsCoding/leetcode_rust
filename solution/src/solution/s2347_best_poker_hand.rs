#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/best-poker-hand/
pub struct Solution;

impl Solution {
    pub fn best_hand(ranks: Vec<i32>, suits: Vec<char>) -> String {
        let first_suit = suits[0];
        if suits.into_iter().all(|s| s == first_suit) {
            "Flush"
        } else {
            let counter = ranks
                .iter()
                .fold(std::collections::HashMap::new(), |mut acc, &rank| {
                    acc.entry(rank).and_modify(|v| *v += 1).or_insert(1);
                    acc
                });

            if counter.values().any(|&v| v >= 3) {
                "Three of a Kind"
            } else if counter.values().any(|&v| v >= 2) {
                "Pair"
            } else {
                "High Card"
            }
        }
        .into()
    }
}

#[cfg(test)]
mod tests {
    use crate::vec_char;

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "Flush".to_string(),
            Solution::best_hand(vec![13, 2, 3, 1, 9], vec_char!["a", "a", "a", "a", "a"])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "Three of a Kind".to_string(),
            Solution::best_hand(vec![4, 4, 2, 4, 4], vec_char!["d", "a", "a", "b", "c"])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            "Pair".to_string(),
            Solution::best_hand(vec![10, 10, 2, 12, 9], vec_char!["a", "b", "c", "a", "d"])
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            "Pair".to_string(),
            Solution::best_hand(vec![2, 10, 7, 10, 7], vec_char!["a", "b", "a", "d", "b"])
        );
    }
}
