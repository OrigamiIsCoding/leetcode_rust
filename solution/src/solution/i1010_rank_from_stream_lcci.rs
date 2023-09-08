#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/rank-from-stream-lcci/
use crate::data_struct_impl::FenwickTree;

struct StreamRank {
    container: FenwickTree<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StreamRank {
    fn new() -> Self {
        Self {
            container: FenwickTree::new(50010),
        }
    }

    fn track(&mut self, x: i32) {
        self.container.add(x as usize + 1, 1);
    }

    fn get_rank_of_number(&self, x: i32) -> i32 {
        self.container.query(x as usize + 1)
    }
}

/**
 * Your StreamRank object will be instantiated and called as such:
 * let obj = StreamRank::new();
 * obj.track(x);
 * let ret_2: i32 = obj.get_rank_of_number(x);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut sr = StreamRank::new();
        assert_eq!(0, sr.get_rank_of_number(0));
        sr.track(0);
        assert_eq!(1, sr.get_rank_of_number(0));
    }
}
