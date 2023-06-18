#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn mice_and_cheese(reward1: Vec<i32>, reward2: Vec<i32>, k: i32) -> i32 {
        let mut reward: Vec<_> = reward1
            .iter()
            .zip(reward2.iter())
            .collect();
        reward.sort_by(|&a, &b| (a.0 - a.1).cmp(&(b.0 - b.1)));

        let mut ans = 0;
        let mut k = k;
        
        for (r1, r2) in reward.into_iter().rev() {
            if k > 0 {
                k -= 1;
                ans += r1;
            } else {
                ans += r2;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            15,
            Solution::mice_and_cheese(vec![1, 1, 3, 4], vec![4, 4, 1, 1], 2)
        )
    }

    #[test]
    fn test_2() {
        assert_eq!(2, Solution::mice_and_cheese(vec![1, 1], vec![1, 1], 2))
    }
}
