#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/escape-the-spreading-fire/
pub struct Solution;

const DIRECTION: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

impl Solution {
    pub fn maximum_minutes(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = Self::fire_spread(grid);
        let (n, m) = (grid.len() as i32, grid[0].len() as i32);

        let mut queue = std::collections::VecDeque::new();
        queue.push_back((0, 0, 1));

        let ans = -1;
        while let Some((x, y, timestamp)) = queue.pop_front() {
            let fire_timestamp = -grid[x][y];
            grid[x][y] = i32::MAX / 2;

            for (dx, dy) in DIRECTION {
                let (x, y) = (x as i32 + dx, y as i32 + dy);

                if x >= 0 && x < n && y >= 0 && y < m {
                    let (x, y) = (x as usize, y as usize);

                    if timestamp + 1 < -grid[x][y] || grid[x][y] == 0 {
                        if x as i32 == n - 1 && y as i32 == m - 1 {
                            if grid[n as usize - 1][m as usize - 1] == 0 {
                                return 1e9 as i32;
                            }
                            return i32::max(ans, fire_timestamp - timestamp - 1);
                        }
                        queue.push_back((x, y, timestamp + 1));
                    }
                }
            }
        }

        ans
    }
    fn fire_spread(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (n, m) = (grid.len() as i32, grid[0].len() as i32);
        let mut queue = std::collections::VecDeque::new();

        for (x, row) in grid.iter().enumerate() {
            for (y, &val) in row.iter().enumerate() {
                if val == 1 {
                    queue.push_back((x, y, -1));
                }
            }
        }

        while let Some((x, y, prev)) = queue.pop_front() {
            grid[x][y] = prev;
            for (dx, dy) in DIRECTION {
                let (x, y) = (x as i32 + dx, y as i32 + dy);

                if x >= 0 && x < n && y >= 0 && y < m && grid[x as usize][y as usize] == 0 {
                    queue.push_back((x as usize, y as usize, prev - 1));
                }
            }
        }

        grid
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
            3,
            Solution::maximum_minutes(mat![
                [0, 2, 0, 0, 0, 0, 0],
                [0, 0, 0, 2, 2, 1, 0],
                [0, 2, 0, 0, 1, 2, 0],
                [0, 0, 2, 2, 2, 0, 2],
                [0, 0, 0, 0, 0, 0, 0]
            ])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            -1,
            Solution::maximum_minutes(mat![[0, 0, 0, 0], [0, 1, 2, 0], [0, 2, 0, 0]])
        )
    }

    #[test]
    fn test_3() {
        assert_eq!(
            1e9 as i32,
            Solution::maximum_minutes(mat![[0, 0, 0], [2, 2, 0], [1, 2, 0]])
        )
    }

    #[test]
    fn test_4() {
        assert_eq!(
            0,
            Solution::maximum_minutes(mat![
                [0, 2, 0, 0, 1],
                [0, 2, 0, 2, 2],
                [0, 2, 0, 0, 0],
                [0, 0, 2, 2, 0],
                [0, 0, 0, 0, 0]
            ])
        )
    }

    #[test]
    fn test_5() {
        assert_eq!(
            1,
            Solution::maximum_minutes(mat![
                [0, 0, 0, 0, 0],
                [0, 2, 0, 2, 0],
                [0, 2, 0, 2, 0],
                [0, 2, 1, 2, 0],
                [0, 2, 2, 2, 0],
                [0, 0, 0, 0, 0]
            ])
        )
    }
}
