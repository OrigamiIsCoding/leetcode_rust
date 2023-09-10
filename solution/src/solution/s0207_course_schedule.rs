#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/course-schedule/
pub struct Solution;

impl Solution {
    pub fn can_finish(mut num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let (mut in_count, mut edges) = (
            vec![0; num_courses as usize],
            vec![Vec::new(); num_courses as usize],
        );

        for prerequire in prerequisites {
            let [a, b] = prerequire[..] else { panic!() };
            in_count[a as usize] += 1;
            edges[b as usize].push(a as usize);
        }

        let mut queue: std::collections::VecDeque<_> = in_count
            .iter()
            .enumerate()
            .filter_map(|(i, &cnt)| if cnt == 0 { Some(i) } else { None })
            .collect();

        while let Some(front) = queue.pop_front() {
            num_courses -= 1;

            for &next in edges[front].iter() {
                in_count[next] -= 1;
                if in_count[next] == 0 {
                    queue.push_back(next);
                }
            }
        }

        num_courses == 0
    }
}

#[cfg(test)]
mod tests {
    use crate::mat;

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::can_finish(2, mat![[1, 0]]));
    }

    #[test]
    fn test_2() {
        assert_eq!(false, Solution::can_finish(2, mat![[1, 0], [0, 1]]));
    }
}
