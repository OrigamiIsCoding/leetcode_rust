#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/course-schedule-ii/
pub struct Solution;

impl Solution {
    pub fn find_order(mut num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let (mut in_count, mut edges, mut result) = (
            vec![0; num_courses as usize],
            vec![Vec::new(); num_courses as usize],
            Vec::new(),
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
            result.push(front as i32);

            for &next in edges[front].iter() {
                in_count[next] -= 1;
                if in_count[next] == 0 {
                    queue.push_back(next);
                }
            }
        }

        if num_courses == 0 {
            result
        } else {
            Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mat;

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![0, 1], Solution::find_order(2, mat![[1, 0]]));
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![0, 1, 2, 3],
            Solution::find_order(4, mat![[1, 0], [2, 0], [3, 1], [3, 2]])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(vec![0], Solution::find_order(1, mat![]));
    }
}
