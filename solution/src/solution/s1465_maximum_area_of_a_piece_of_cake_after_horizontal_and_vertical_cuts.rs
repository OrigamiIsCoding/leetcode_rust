#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/maximum-area-of-a-piece-of-cake-after-horizontal-and-vertical-cuts/
pub struct Solution;

impl Solution {
    pub fn max_area(
        h: i32,
        w: i32,
        mut horizontal_cuts: Vec<i32>,
        mut vertical_cuts: Vec<i32>,
    ) -> i32 {
        fn find_longest_span(mut nums: Vec<i32>) -> i64 {
            nums.sort();
            nums.windows(2).map(|w| (w[1] - w[0]) as i64).max().unwrap()
        }
        horizontal_cuts.push(0);
        horizontal_cuts.push(h);
        vertical_cuts.push(0);
        vertical_cuts.push(w);
        let h = find_longest_span(horizontal_cuts);
        let w = find_longest_span(vertical_cuts);
        ((h * w) % 1_000_000_007) as i32
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, Solution::max_area(5, 4, vec![1, 2, 4], vec![1, 3]));
    }

    #[test]
    fn test_2() {
        assert_eq!(6, Solution::max_area(5, 4, vec![3, 1], vec![1]));
    }

    #[test]
    fn test_3() {
        assert_eq!(9, Solution::max_area(5, 4, vec![3], vec![3]));
    }
}
