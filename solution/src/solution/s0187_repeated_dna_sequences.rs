#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/repeated-dna-sequences/
pub struct Solution;

use std::str;
#[derive(Default)]
struct Trie([Option<Box<Trie>>; 4]);

impl Trie {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, seq: &[u8]) {
        (0..10).fold(self, |node, k| {
            node.0[Self::get_id(seq[k])].get_or_insert(Default::default())
        });
    }

    fn query(&self, seq: &[u8]) -> bool {
        let mut node = self;
        for i in 0..10 {
            let next = Self::get_id(seq[i]);
            if let Some(next) = &node.0[next] {
                node = next;
            } else {
                return false;
            }
        }
        true
    }

    fn get_id(value: u8) -> usize {
        match value {
            b'A' => 0,
            b'T' => 1,
            b'C' => 2,
            _ => 3,
        }
    }
}

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let mut trie = Trie::new();
        let s = s.into_bytes();
        let h: std::collections::HashSet<_> = s
            .windows(10)
            .filter(|seq| {
                if trie.query(seq) {
                    true
                } else {
                    trie.insert(seq);
                    false
                }
            })
            .collect();
        h.into_iter()
            .map(|v| str::from_utf8(v).unwrap().into())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec!["AAAAACCCCC", "CCCCCAAAAA"],
            Solution::find_repeated_dna_sequences("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".into())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec!["AAAAAAAAAA"],
            Solution::find_repeated_dna_sequences("AAAAAAAAAAAAA".into())
        );
    }
}
