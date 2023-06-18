#![allow(dead_code)]

type Vec2<T> = Vec<Vec<T>>;
use std::collections::VecDeque;

#[derive(Debug)]
struct TreeAncestor {
    fa: Vec2<usize>,
    k: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TreeAncestor {
    fn lca_build(g: Vec2<usize>) -> Self {
        let k = (g.len() as f64).log2().ceil() as usize;
        let mut fa = vec![vec![0; k]; g.len()];

        let mut queue = VecDeque::new();
        queue.push_back(1);

        // BFS
        while let Some(u) = queue.pop_front() {
            // foreach neighborhood
            for &i in g[u].iter() {
                queue.push_back(i);
                fa[i][0] = u;
                for j in 1..k {
                    fa[i][j] = fa[fa[i][j - 1]][j - 1];
                }
            }
        }

        Self { fa, k }
    }

    fn new(n: i32, parent: Vec<i32>) -> Self {
        let mut g: Vec2<usize> = vec![Vec::new(); (n + 1) as usize];

        for (a, b) in parent.into_iter().enumerate().skip(1) {
            // add edge b -> a
            g[(b + 1) as usize].push(a + 1);
        }

        // lca
        Self::lca_build(g)
    }

    fn get_kth_ancestor(&self, node: i32, k: i32) -> i32 {
        let mut node = node + 1;
        for i in 0..self.k {
            if k >> i & 1 == 1 {
                node = self.fa[node as usize][i] as i32;
            }
        }
        node - 1
    }
}

/**
 * Your TreeAncestor object will be instantiated and called as such:
 * let obj = TreeAncestor::new(n, parent);
 * let ret_1: i32 = obj.get_kth_ancestor(node, k);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let t = TreeAncestor::new(7, vec![-1, 0, 0, 1, 1, 2, 2]);
        assert_eq!(1, t.get_kth_ancestor(3, 1));
        assert_eq!(0, t.get_kth_ancestor(5, 2));
        assert_eq!(-1, t.get_kth_ancestor(6, 3));
    }
}
