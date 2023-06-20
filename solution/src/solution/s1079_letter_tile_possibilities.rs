#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/letter-tile-possibilities/description/
pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let mut counter = HashMap::with_capacity(26);
        tiles
            .bytes()
            .for_each(|b| *counter.entry(b).or_insert(0) += 1);

        let mut counter: Vec<_> = counter.into_values().collect();

        fn dfs(counter: &mut Vec<i32>) -> i32 {
            let mut ans = 0;

            for i in 0..counter.len() {
                if counter[i] > 0 {
                    counter[i] -= 1;
                    ans += dfs(counter) + 1;
                    counter[i] += 1;
                }
            }

            ans
        }

        dfs(&mut counter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(8, Solution::num_tile_possibilities("AAB".into()));
    }

    #[test]
    fn test_2() {
        assert_eq!(188, Solution::num_tile_possibilities("AAABBC".into()));
    }

    #[test]
    fn test_3() {
        assert_eq!(1, Solution::num_tile_possibilities("V".into()));
    }
}
