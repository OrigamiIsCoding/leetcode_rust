#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/pond-sizes-lcci/
pub struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn pond_sizes(mut land: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = Vec::new();
        let (n, m) = (land.len(), land[0].len());

        let bfs = |land: &mut Vec<Vec<i32>>, x: usize, y: usize| -> i32 {
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
            let (n, m) = (land.len() as i32, land[0].len() as i32);
            let mut queue = VecDeque::new();
            let mut count = 0;
            queue.push_back((x as i32, y as i32));
            land[x][y] = 1;
            while let Some((x, y)) = queue.pop_front() {
                count += 1;

                for (dx, dy) in D {
                    let x = x + dx;
                    let y = y + dy;
                    if x >= 0 && x < n && y >= 0 && y < m && land[x as usize][y as usize] == 0 {
                        land[x as usize][y as usize] = 1;
                        queue.push_back((x, y));
                    }
                }
            }
            count
        };

        for i in 0..n {
            for j in 0..m {
                if land[i][j] == 0 {
                    ans.push(bfs(&mut land, i, j));
                }
            }
        }
        ans.sort();
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
            vec![1, 2, 4],
            Solution::pond_sizes(mat![[0, 2, 1, 0], [0, 1, 0, 1], [1, 1, 0, 1], [0, 1, 0, 1]])
        );
    }
}
