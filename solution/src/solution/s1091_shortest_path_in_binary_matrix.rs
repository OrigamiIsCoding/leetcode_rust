#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/shortest-path-in-binary-matrix/
pub struct Solution;

impl Solution {
    pub fn shortest_path_binary_matrix(mut grid: Vec<Vec<i32>>) -> i32 {
        if grid[0][0] == 1 {
            return -1;
        }
        const D: [(i32, i32); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        let n = grid.len() as i32;
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((0, 0, 1));
        while let Some((x, y, mut cost)) = queue.pop_front() {
            if x == n - 1 && y == n - 1 {
                return cost;
            }
            cost += 1;
            for (dx, dy) in D {
                let (x, y) = (x + dx, y + dy);
                if (0..n).contains(&x) && (0..n).contains(&y) && grid[x as usize][y as usize] == 0 {
                    grid[x as usize][y as usize] = 1;
                    queue.push_back((x, y, cost));
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            2,
            Solution::shortest_path_binary_matrix(vec![vec![0, 1], vec![1, 0]])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            4,
            Solution::shortest_path_binary_matrix(vec![
                vec![0, 0, 0],
                vec![1, 1, 0],
                vec![1, 1, 0],
            ])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            -1,
            Solution::shortest_path_binary_matrix(vec![
                vec![1, 0, 0],
                vec![1, 1, 0],
                vec![1, 1, 0],
            ])
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            -1,
            Solution::shortest_path_binary_matrix(vec![
                vec![0, 0, 0],
                vec![1, 1, 0],
                vec![1, 1, 1],
            ])
        );
    }

    #[test]
    fn test_5() {
        assert_eq!(1, Solution::shortest_path_binary_matrix(vec![vec![0]]))
    }
}
