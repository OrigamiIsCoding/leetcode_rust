#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/find-eventual-safe-states/
pub struct Solution;

impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        fn dfs(
            graph: &Vec<Vec<i32>>,
            st: &mut Vec<Option<bool>>,
            st2: &mut Vec<bool>,
            u: usize,
        ) -> bool {
            st2[u] = true;
            if let Some(t) = st[u] {
                return t;
            }

            for &ne in graph[u].iter() {
                if st2[ne as usize] {
                    continue;
                }
                if !dfs(graph, st, st2, ne as usize) {
                    st[u] = Some(false);
                    return false;
                }
            }

            st[u] = Some(true);
            true
        }

        let n = graph.len();
        let mut st = vec![None; n];
        let mut ans = Vec::new();
        for i in 0..n {
            let mut st2 = vec![false; n];
            if dfs(&graph, &mut st, &mut st2, i) {
                ans.push(i as i32);
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::mat;

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![2, 4, 5, 6],
            Solution::eventual_safe_nodes(mat![[1, 2], [2, 3], [5], [0], [5], [], []])
        );
    }
}
