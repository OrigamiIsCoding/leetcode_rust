#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/reward-top-k-students/
pub struct Solution;

impl Solution {
    pub fn top_students(
        positive_feedback: Vec<String>,
        negative_feedback: Vec<String>,
        report: Vec<String>,
        student_id: Vec<i32>,
        k: i32,
    ) -> Vec<i32> {
        let positive_feedback: std::collections::HashSet<_> =
            positive_feedback.into_iter().collect();
        let negative_feedback: std::collections::HashSet<_> =
            negative_feedback.into_iter().collect();
        let mut students: Vec<_> = report
            .into_iter()
            .zip(student_id)
            .map(|(report, student_id)| {
                let score = report
                    .split_whitespace()
                    .filter_map(|word| {
                        if positive_feedback.contains(word) {
                            Some(3)
                        } else if negative_feedback.contains(word) {
                            Some(-1)
                        } else {
                            None
                        }
                    })
                    .sum::<i32>();
                (-score, student_id)
            })
            .collect();
        students.sort();
        students
            .into_iter()
            .map(|(_, id)| id)
            .take(k as usize)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]

    use super::*;
    use crate::vec_into;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![1, 2],
            Solution::top_students(
                vec_into!["smart", "brilliant", "studious"],
                vec_into!["not"],
                vec_into!["this student is studious", "the student is smart"],
                vec![1, 2],
                2
            )
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![2, 1],
            Solution::top_students(
                vec_into!["smart", "brilliant", "studious"],
                vec_into!["not"],
                vec_into!["this student is not studious", "the student is smart"],
                vec![1, 2],
                2
            )
        );
    }
}
