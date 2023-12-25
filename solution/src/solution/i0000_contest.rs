#![allow(dead_code)]
/// reference: https://leetcode.cn/contest/
pub struct Solution;

impl Solution {
    pub fn maximum_score_after_operations(edges: Vec<Vec<i32>>, values: Vec<i32>) -> i64 {
        fn dfs(edges: &Vec<Vec<usize>>, values: &Vec<i32>, u: usize, fa: usize, flag: bool) -> i64 {
            println!("{u} {fa}");
            let mut sum1 = values[u] as i64;
            let mut sum2 = 0;
            for &j in edges[u].iter() {
                if j != fa {
                    let r = dfs(edges, values, j, u, true);
                    if !flag {
                        sum1 += dfs(edges, values, j, u, false);
                    } else {
                        sum1 += r;
                    }
                    sum2 += r;
                }
            }
            if sum2 == 0 {
                i64::MIN
            } else {
                i64::max(sum1, sum2)
            }
        }
        let mut next = vec![Vec::new(); values.len()];
        for edge in edges {
            next[edge[0] as usize].push(edge[1] as usize);
        }
        dfs(&next, &values, 0, values.len(), false)
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]

    use super::*;
    use crate::mat;

    #[test]
    fn test_1() {
        assert_eq!(
            11,
            Solution::maximum_score_after_operations(
                mat![[0, 1], [0, 2], [0, 3], [2, 4], [4, 5]],
                vec![5, 2, 5, 2, 1, 1]
            )
        );
    }
}
