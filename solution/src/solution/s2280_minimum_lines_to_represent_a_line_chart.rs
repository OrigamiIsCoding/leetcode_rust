#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/minimum-lines-to-represent-a-line-chart/
pub struct Solution;

impl Solution {
    pub fn minimum_lines(mut stock_prices: Vec<Vec<i32>>) -> i32 {
        if stock_prices.len() <= 2 {
            return (stock_prices.len() - 1) as i32;
        }
        stock_prices.sort();

        stock_prices
            .windows(3)
            .filter(|window| {
                let (left, mid, right) = (&window[0], &window[1], &window[2]);

                let dx0 = (mid[0] - left[0]) as i64;
                let dy0 = (mid[1] - left[1]) as i64;
                let dx1 = (right[0] - mid[0]) as i64;
                let dy1 = (right[1] - mid[1]) as i64;

                dx0 * dy1 != dy0 * dx1
            })
            .count() as i32
            + 1
    }
}

#[cfg(test)]
mod tests {
    use crate::mat;

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            3,
            Solution::minimum_lines(mat![
                [1, 7],
                [2, 6],
                [3, 5],
                [4, 4],
                [5, 4],
                [6, 3],
                [7, 2],
                [8, 1]
            ])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            1,
            Solution::minimum_lines(mat![[3, 4], [1, 2], [7, 8], [2, 3]])
        );
    }
}
