#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/remove-letter-to-equalize-frequency/
pub struct Solution;

impl Solution {
    pub fn equal_frequency(word: String) -> bool {
        let mut h = std::collections::HashMap::new();
        word.into_bytes().iter().for_each(|&b| {
            h.entry(b).and_modify(|cnt| *cnt += 1).or_insert(1);
        });

        h.keys().any(|&key| {
            let counter: std::collections::HashSet<_> = h
                .iter()
                .filter_map(|(&k, &v)| match v - 1 {
                    0 if k == key => None,
                    v if k == key => Some(v),
                    _ => Some(v),
                })
                .collect();
            counter.len() <= 1
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::equal_frequency("abcc".into()));
    }

    #[test]
    fn test_2() {
        assert_eq!(false, Solution::equal_frequency("aazz".into()));
    }
}
