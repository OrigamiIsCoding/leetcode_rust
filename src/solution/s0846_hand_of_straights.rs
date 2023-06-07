#![allow(dead_code)]
pub struct Solution;

use std::collections::BTreeMap;
impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        if hand.len() as i32 % group_size != 0 {
            return false;
        }
        let mut h = BTreeMap::new();

        for ele in hand {
            let count = h.entry(ele).or_insert(0);
            *count += 1;
        }

        loop {
            match h.iter().next() {
                None => break,
                Some((&first, _)) => {
                    for i in first..(first+ group_size) {
                        match h.get_mut(&i) {
                            Some(count) => {
                                *count -= 1;
                                if *count == 0 {
                                    h.remove(&i).unwrap();
                                }
                            }
                            None => return false,
                        }
                    }
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            true,
            Solution::is_n_straight_hand(vec![1, 2, 3, 6, 2, 3, 4, 7, 8], 3)
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(false, Solution::is_n_straight_hand(vec![1, 2, 3, 4, 5], 4));
    }
}
