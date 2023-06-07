#![allow(dead_code)]
pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut count = HashMap::new();
        for b in s.bytes() {
            *(count.entry(b).or_insert(0)) += 1;
        }
        for b in t.bytes() {
            match count.get_mut(&b) {
                None => return false,
                Some(v) => (*v) -= 1,
            }
        }
        count.into_values().all(|v| v == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            true,
            Solution::is_anagram("anagram".into(), "anagram".into())
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(false, Solution::is_anagram("rat".into(), "cat".into()));
    }
}
