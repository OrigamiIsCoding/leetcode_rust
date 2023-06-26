#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/find-original-array-from-doubled-array/
pub struct Solution;

impl Solution {
    pub fn find_original_array(mut changed: Vec<i32>) -> Vec<i32> {
        if changed.len() % 2 != 0 {
            return Vec::new();
        }
        changed.sort();
        let mut h = std::collections::HashMap::new();
        changed.iter().rev().for_each(|&v| {
            *h.entry(v).or_insert(0) += 1;
        });

        let mut result = Vec::new();
        for v in changed {
            match h.get_mut(&v) {
                Some(cnt) if *cnt > 0 => *cnt -= 1,
                _ => continue,
            }
            match h.get_mut(&(v * 2)) {
                Some(cnt) if *cnt > 0 => {
                    *cnt -= 1;
                    result.push(v);
                }
                _ => return Vec::new(),
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![1, 3, 4],
            Solution::find_original_array(vec![1, 3, 4, 2, 6, 8])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Vec::<i32>::new(),
            Solution::find_original_array(vec![6, 3, 0, 1])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(Vec::<i32>::new(), Solution::find_original_array(vec![1]));
    }

    #[test]
    fn test_4() {
        assert_eq!(vec![0, 0], Solution::find_original_array(vec![0, 0, 0, 0]));
    }

    #[test]
    fn test_5() {
        assert_eq!(
            Vec::<i32>::new(),
            Solution::find_original_array(vec![1, 2, 1, 0])
        );
    }
}
