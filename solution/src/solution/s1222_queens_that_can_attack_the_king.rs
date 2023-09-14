#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/queens-that-can-attack-the-king/
pub struct Solution;

impl Solution {
    const D: [(i32, i32); 8] = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    pub fn queens_attackthe_king(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
        let h: std::collections::HashSet<_> = queens.into_iter().map(|v| (v[0], v[1])).collect();
        let mut ans = Vec::new();
        let [x, y] = king[..] else { panic!() };
        for (dx, dy) in Self::D {
            for idx in (0..8).map_while(|i| {
                let (nx, ny) = (x + i * dx, y + i * dy);
                if (0..8).contains(&nx) && (0..8).contains(&ny) {
                    Some((nx, ny))
                } else {
                    None
                }
            }) {
                if h.contains(&idx) {
                    ans.push(vec![idx.0, idx.1]);
                    break;
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_ignored_order, mat};

    use super::*;

    #[test]
    fn test_1() {
        assert_ignored_order!(
            mat![[0, 1], [1, 0], [3, 3]],
            Solution::queens_attackthe_king(
                mat![[0, 1], [1, 0], [4, 0], [0, 4], [3, 3], [2, 4]],
                vec![0, 0]
            )
        );
    }

    #[test]
    fn test_2() {
        assert_ignored_order!(
            mat![[2, 2], [3, 4], [4, 4]],
            Solution::queens_attackthe_king(
                mat![[0, 0], [1, 1], [2, 2], [3, 4], [3, 5], [4, 4], [4, 5]],
                vec![3, 3]
            )
        );
    }

    #[test]
    fn test_3() {
        assert_ignored_order!(
            mat![[2, 3], [1, 4], [1, 6], [3, 7], [4, 3], [5, 4], [4, 5]],
            Solution::queens_attackthe_king(
                mat![
                    [5, 6],
                    [7, 7],
                    [2, 1],
                    [0, 7],
                    [1, 6],
                    [5, 1],
                    [3, 7],
                    [0, 3],
                    [4, 0],
                    [1, 2],
                    [6, 3],
                    [5, 0],
                    [0, 4],
                    [2, 2],
                    [1, 1],
                    [6, 4],
                    [5, 4],
                    [0, 0],
                    [2, 6],
                    [4, 5],
                    [5, 2],
                    [1, 4],
                    [7, 5],
                    [2, 3],
                    [0, 5],
                    [4, 2],
                    [1, 0],
                    [2, 7],
                    [0, 1],
                    [4, 6],
                    [6, 1],
                    [0, 6],
                    [4, 3],
                    [1, 7]
                ],
                vec![3, 4]
            )
        );
    }
}
