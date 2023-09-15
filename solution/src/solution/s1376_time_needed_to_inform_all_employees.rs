#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/time-needed-to-inform-all-employees/
pub struct Solution;

impl Solution {
    pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        let mut edges = vec![Vec::new(); n as usize];
        manager.into_iter().enumerate().for_each(|(i, v)| {
            if v != -1 {
                edges[v as usize].push(i);
            }
        });
        let mut ans = 0;
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((head_id as usize, 0));
        while let Some((u, cost)) = queue.pop_front() {
            ans = i32::max(ans, cost);
            for &next in edges[u].iter() {
                queue.push_back((next, cost + inform_time[u]));
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(0, Solution::num_of_minutes(1,0,vec![-1],vec![0]));
    }
    #[test]
    fn test_2() {
        assert_eq!(1, Solution::num_of_minutes(6, 2, vec![2,2,-1,2,2,2], vec![0,0,1,0,0,0]));
    }
}
