#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/maximum-xor-of-two-numbers-in-an-array/
pub struct Solution;

use std::ops::{Index, IndexMut};
struct Node {
    next: [usize; 2],
}

impl Node {
    fn new() -> Node {
        Node { next: [0; 2] }
    }
}

impl Index<usize> for Node {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        self.next.index(index)
    }
}

impl IndexMut<usize> for Node {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.next.index_mut(index)
    }
}

struct Trie {
    nodes: Vec<Node>,
}

impl Trie {
    fn new() -> Trie {
        Trie {
            nodes: vec![Node::new()],
        }
    }

    fn insert(&mut self, num: i32) {
        let mut p = 0;
        for i in (0..32).rev() {
            let v = (num >> i & 1) as usize;
            if self.nodes[p][v] == 0 {
                self.nodes[p][v] = self.push();
            }
            p = self.nodes[p][v];
        }
    }

    fn query(&self, num: i32) -> i32 {
        let mut p = 0;
        let mut result = 0;
        for i in (0..32).rev() {
            let v = (num >> i & 1) as usize;
            match self.nodes[p][v ^ 1] {
                0 => p = self.nodes[p][v],
                _ => {
                    result |= 1 << i;
                    p = self.nodes[p][v ^ 1];
                }
            }
        }
        result
    }

    fn push(&mut self) -> usize {
        self.nodes.push(Node::new());
        self.nodes.len() - 1
    }
}

impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let mut trie = Trie::new();
        nums.into_iter().fold(0, |acc, v| {
            let r = trie.query(v);
            trie.insert(v);
            i32::max(acc, r)
        })
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(28, Solution::find_maximum_xor(vec![3, 10, 5, 25, 2, 8]));
    }

    #[test]
    fn test_2() {
        assert_eq!(
            127,
            Solution::find_maximum_xor(vec![14, 70, 53, 83, 49, 91, 36, 80, 92, 51, 66, 70])
        );
    }
}
