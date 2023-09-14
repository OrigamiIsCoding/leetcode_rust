#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/course-schedule-iv/
pub struct Solution;

impl Solution {
    pub fn check_if_prerequisite(
        num_courses: i32,
        prerequisites: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        const INF: i32 = 0x3f3f3f3f;
        let num_courses = num_courses as usize;
        let mut distance = vec![vec![INF; num_courses]; num_courses];

        for prerequire in prerequisites {
            let [a, b] = prerequire[..] else {panic!()};
            distance[b as usize][a as usize] = 1;
        }

        for k in 0..num_courses {
            for i in 0..num_courses {
                for j in 0..num_courses {
                    distance[i][j] = i32::min(distance[i][j], distance[i][k] + distance[k][j]);
                }
            }
        }

        queries
            .into_iter()
            .map(|q| distance[q[1] as usize][q[0] as usize] != INF)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::mat;

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![false, true],
            Solution::check_if_prerequisite(2, mat![[1, 0]], mat![[0, 1], [1, 0]])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![false, false],
            Solution::check_if_prerequisite(2, mat![], mat![[0, 1], [1, 0]])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            vec![true, true],
            Solution::check_if_prerequisite(3, mat![[1, 2], [1, 0], [2, 0]], mat![[1, 0], [1, 2]])
        );
    }
}
