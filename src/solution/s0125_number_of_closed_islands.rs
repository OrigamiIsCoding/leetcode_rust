#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/number-of-closed-islands/
pub struct Solution;

impl Solution {
    const D: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    fn check(grid: &mut Vec<Vec<i32>>, x: usize, y: usize) -> bool {
        grid[x][y] = 2;
        let (n, m) = (grid.len(), grid[0].len());
        let mut result = false;
        for (dx, dy) in Self::D {
            let x = x as i32 + dx;
            let y = y as i32 + dy;

            if x >= 0 && x < n as i32 && y >= 0 && y < m as i32 && grid[x as usize][y as usize] == 0
            {
                result |= Self::check(grid, x as usize, y as usize);
            }
        }
        if x == 0 || y == 0 || x == n - 1 || y == m - 1 {
            return true;
        }
        result
    }

    pub fn closed_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == 0 && !Self::check(&mut grid, i, j) {
                    ans += 1;
                }
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
            2,
            Solution::closed_island(mat![
                [1, 1, 1, 1, 1, 1, 1, 0],
                [1, 0, 0, 0, 0, 1, 1, 0],
                [1, 0, 1, 0, 1, 1, 1, 0],
                [1, 0, 0, 0, 0, 1, 0, 1],
                [1, 1, 1, 1, 1, 1, 1, 0]
            ])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            1,
            Solution::closed_island(mat![[0, 0, 1, 0, 0], [0, 1, 0, 1, 0], [0, 1, 1, 1, 0]])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            2,
            Solution::closed_island(mat![
                [1, 1, 1, 1, 1, 1, 1],
                [1, 0, 0, 0, 0, 0, 1],
                [1, 0, 1, 1, 1, 0, 1],
                [1, 0, 1, 0, 1, 0, 1],
                [1, 0, 1, 1, 1, 0, 1],
                [1, 0, 0, 0, 0, 0, 1],
                [1, 1, 1, 1, 1, 1, 1]
            ])
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            4,
            Solution::closed_island(mat![
                [1, 1, 0, 1, 1, 1, 1, 1, 1, 1],
                [0, 0, 1, 0, 0, 1, 0, 1, 1, 1],
                [1, 0, 1, 0, 0, 0, 1, 0, 1, 0],
                [1, 1, 1, 1, 1, 0, 0, 1, 0, 0],
                [1, 0, 1, 0, 1, 1, 1, 1, 1, 0],
                [0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
                [1, 0, 1, 0, 0, 0, 0, 1, 1, 0],
                [1, 1, 0, 0, 1, 1, 0, 0, 0, 0],
                [0, 0, 0, 1, 1, 0, 1, 1, 1, 0],
                [1, 1, 0, 1, 0, 1, 0, 0, 1, 0]
            ])
        )
    }
}
