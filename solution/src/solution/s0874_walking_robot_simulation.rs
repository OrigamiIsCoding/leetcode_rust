#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/walking-robot-simulation/
pub struct Solution;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
    fn distance(self) -> i32 {
        self.x * self.x + self.y * self.y
    }
}

impl From<Vec<i32>> for Point {
    fn from(value: Vec<i32>) -> Self {
        Self::new(value[0], value[1])
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Bottom,
    Left,
    Right,
}

impl Direction {
    fn turn_left(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Bottom => Self::Right,
            Self::Left => Self::Bottom,
            Self::Right => Self::Up,
        }
    }
    fn turn_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Bottom,
            Self::Bottom => Self::Left,
            Self::Left => Self::Up,
        }
    }
    fn next(self, p: &Point) -> Point {
        let (x, y) = (p.x, p.y);
        match self {
            Self::Up => Point::new(x, y + 1),
            Self::Right => Point::new(x + 1, y),
            Self::Left => Point::new(x - 1, y),
            Self::Bottom => Point::new(x, y - 1),
        }
    }
}

impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let h: std::collections::HashSet<Point> = obstacles.into_iter().map(|v| v.into()).collect();
        let mut ans = 0;
        let mut current = Point::new(0, 0);
        let mut direction = Direction::Up;

        for command in commands {
            match command {
                -2 => direction = direction.turn_left(),
                -1 => direction = direction.turn_right(),
                d => {
                    for _ in 0..d {
                        let next = direction.next(&current);
                        if h.contains(&next) {
                            break;
                        }
                        current = next;
                    }
                    ans = i32::max(ans, current.distance());
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
        assert_eq!(25, Solution::robot_sim(vec![4, -1, 3], mat![]));
    }

    #[test]
    fn test_2() {
        assert_eq!(65, Solution::robot_sim(vec![4, -1, 4, -2, 4], mat![[2, 4]]));
    }

    #[test]
    fn test_3() {
        assert_eq!(36, Solution::robot_sim(vec![6, -1, -1, 6], mat![]));
    }
}
