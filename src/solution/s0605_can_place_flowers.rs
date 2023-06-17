#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/can-place-flowers/
pub struct Solution;

impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, n: i32) -> bool {
        let mut count = 0;
        for i in 0..flowerbed.len() {
            let left = if i == 0 {
                0
            } else {
                *flowerbed.get(i - 1).unwrap_or(&0)
            };
            let mid = flowerbed[i];
            let right = *flowerbed.get(i + 1).unwrap_or(&0);

            if left + mid + right == 0 {
                flowerbed[i] = 1;
                count += 1;
            }
        }
        count >= n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(true, Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1));
    }

    #[test]
    fn test_2() {
        assert_eq!(false, Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 2));
    }

    #[test]
    fn test_3() {
        assert_eq!(
            true,
            Solution::can_place_flowers(vec![1, 0, 0, 0, 0, 0, 1], 2)
        )
    }

    #[test]
    fn test_4() {
        assert_eq!(
            false,
            Solution::can_place_flowers(vec![1, 0, 0, 0, 0, 1], 2)
        )
    }
}
