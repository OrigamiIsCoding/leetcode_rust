#![allow(dead_code)]

/// reference: https://leetcode.cn/problems/smallest-missing-genetic-value-in-each-subtree/
pub struct Solution;

impl Solution {
    fn find(p: &mut Vec<usize>, x: usize) -> usize {
        if x != p[x] {
            p[x] = Self::find(p, p[x]);
        }
        p[x]
    }

    pub fn smallest_missing_value_subtree(parents: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        let n = parents.len();
        let mut ans = vec![0; n];
        let mut p = Vec::with_capacity(n + 2);
        (0..n + 2).for_each(|idx| p.push(idx));
        let mut out = parents
            .iter()
            .filter(|&&v| v != -1)
            .fold(vec![0; n], |mut acc, &v| {
                acc[v as usize] += 1;
                acc
            });

        let mut queue: std::collections::VecDeque<_> = out
            .iter()
            .enumerate()
            .filter(|&(_, &v)| v == 0)
            .map(|(idx, _)| idx)
            .collect();

        while let Some(front) = queue.pop_front() {
            let current_value = nums[front] as usize;
            let next_value = Self::find(&mut p, 1);

            println!("index = {front}, current = {current_value}, next = {next_value}");
            println!("{:?}", p);
            p[current_value] = p[current_value + 1];
            println!("make {current_value} join {}", current_value + 1);
            if next_value != current_value {
                ans[front] = 1;
                println!("assign = 1");
            } else {
                ans[front] = Self::find(&mut p, 1) as i32;
                println!("assign = {}", Self::find(&mut p, 1) as i32);
            }
            match parents.get(front) {
                Some(&parent) if parent != -1 => {
                    out[parent as usize] -= 1;
                    if out[parent as usize] == 0 {
                        queue.push_back(parent as usize);
                    }
                }
                _ => (),
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
        assert_eq!(
            vec![5, 1, 1, 1],
            Solution::smallest_missing_value_subtree(vec![-1, 0, 0, 2], vec![1, 2, 3, 4])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![7, 1, 1, 4, 2, 1],
            Solution::smallest_missing_value_subtree(
                vec![-1, 0, 1, 0, 3, 3],
                vec![5, 4, 6, 2, 1, 3]
            )
        );
    }
}
