#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/defuse-the-bomb/
pub struct Solution;

impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let n = code.len() as i32;
        let func: Box<dyn Fn(_) -> _> = match k.cmp(&0) {
            std::cmp::Ordering::Less => Box::new(|i: i32| -> i32 {
                (k..0)
                    .map(|j| (j + i + n) % n)
                    .map(|j| code[j as usize])
                    .sum()
            }),
            std::cmp::Ordering::Greater => Box::new(|i: i32| -> i32 {
                (1..=k)
                    .map(|j| (j + i + n) % n)
                    .map(|j| code[j as usize])
                    .sum()
            }),
            std::cmp::Ordering::Equal => Box::new(|_: i32| 0),
        };
        (0..n).map(func).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![12, 10, 16, 13], Solution::decrypt(vec![5, 7, 1, 4], 3));
    }

    #[test]
    fn test_2() {
        assert_eq!(vec![0, 0, 0, 0], Solution::decrypt(vec![1, 2, 3, 4], 0));
    }

    #[test]
    fn test_3() {
        assert_eq!(vec![12, 5, 6, 13], Solution::decrypt(vec![2, 4, 9, 3], -2));
    }
}
