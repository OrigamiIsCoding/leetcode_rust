#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/check-knight-tour-configuration/
pub struct Solution;

impl Solution {
    pub fn check_valid_grid(grid: Vec<Vec<i32>>) -> bool {
        if grid[0][0] != 0 {
            false
        } else {
            Self::check_valid_grid_impl(0, 0, &grid, grid.len() as i32)
        }
    }

    const D: [(i32, i32); 8] = [
        (1, 2),
        (2, 1),
        (1, -2),
        (2, -1),
        (-1, 2),
        (-2, 1),
        (-1, -2),
        (-2, -1),
    ];

    fn check_valid_grid_impl(x: i32, y: i32, grid: &Vec<Vec<i32>>, n: i32) -> bool {
        if grid[x as usize][y as usize] == n * n - 1 {
            true
        } else {
            for (dx, dy) in Self::D {
                let (nx, ny) = (x + dx, y + dy);

                if (0..n).contains(&nx)
                    && (0..n).contains(&ny)
                    && grid[x as usize][y as usize] + 1 == grid[nx as usize][ny as usize]
                {
                    return Self::check_valid_grid_impl(nx, ny, grid, n);
                }
            }
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mat;

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            true,
            Solution::check_valid_grid(mat![
                [0, 11, 16, 5, 20],
                [17, 4, 19, 10, 15],
                [12, 1, 8, 21, 6],
                [3, 18, 23, 14, 9],
                [24, 13, 2, 7, 22]
            ])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            false,
            Solution::check_valid_grid(mat![
                [24, 11, 22, 17, 4],
                [21, 16, 5, 12, 9],
                [6, 23, 10, 3, 18],
                [15, 20, 1, 8, 13],
                [0, 7, 14, 19, 2]
            ])
        );
    }
}
