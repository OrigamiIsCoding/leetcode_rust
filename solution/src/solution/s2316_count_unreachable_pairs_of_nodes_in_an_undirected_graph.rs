#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/count-unreachable-pairs-of-nodes-in-an-undirected-graph/
pub struct Solution;

use crate::data_struct_impl::UnionFind;

impl Solution {
    pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        let mut u = UnionFind::new(n as usize);
        edges.into_iter().for_each(|edge| {
            u.union(edge[0] as usize, edge[1] as usize);
        });
        let mut ans = 0;
        for i in 0..n {
            ans += n as i64 - u.get_len(i as usize) as i64;
        }
        ans / 2
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]

    use super::*;
    use crate::mat;

    #[test]
    fn test_1() {
        assert_eq!(0, Solution::count_pairs(3, mat![[0, 1], [0, 2], [1, 2]]));
    }

    #[test]
    fn test_2() {
        assert_eq!(
            14,
            Solution::count_pairs(7, mat![[0, 2], [0, 5], [2, 4], [1, 6], [5, 4]])
        );
    }
}
