#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/maximum-students-taking-exam/
pub struct Solution;

impl Solution {
    pub fn max_students(seats: Vec<Vec<char>>) -> i32 {
        let (n, m) = (seats.len(), seats[0].len());
        // preprocess collect valid state
        let states: Vec<_> = (0..1 << m)
            .filter(|&s| (s & (s << 1)) == 0 && (s & (s >> 1)) == 0)
            .map(|s: i32| (s as usize, s.count_ones() as i32))
            .collect();
        let seats: Vec<_> =
            seats
                .into_iter()
                .map(|row| {
                    row.into_iter().enumerate().fold(0, |acc, (i, ch)| {
                        if ch == '#' {
                            acc | (1 << i)
                        } else {
                            acc
                        }
                    })
                })
                .collect();

        let mut dp = vec![vec![0; states.len()]; n + 2];
        for i in 1..dp.len() {
            for (j, &(a, _)) in states.iter().enumerate() {
                for (k, &(b, count)) in states.iter().enumerate() {
                    if (b & (a << 1)) == 0
                        && (b & (a >> 1)) == 0
                        && (i == dp.len() - 1 || (b & seats[i - 1]) == 0)
                    {
                        dp[i][k] = i32::max(dp[i - 1][j] + count, dp[i][k]);
                    }
                }
            }
        }
        dp.last().unwrap().first().copied().unwrap()
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use crate::mat_char;
    use test_case::test_case;

    #[test]
    fn test_1() {
        assert_eq!(
            4,
            Solution::max_students(mat_char![
                ["#", ".", "#", "#", ".", "#"],
                [".", "#", "#", "#", "#", "."],
                ["#", ".", "#", "#", ".", "#"]
            ])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            3,
            Solution::max_students(mat_char![
                [".", "#"],
                ["#", "#"],
                ["#", "."],
                ["#", "#"],
                [".", "#"]
            ])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            10,
            Solution::max_students(mat_char![
                ["#", ".", ".", ".", "#"],
                [".", "#", ".", "#", "."],
                [".", ".", "#", ".", "."],
                [".", "#", ".", "#", "."],
                ["#", ".", ".", ".", "#"]
            ])
        );
    }
}
