#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/distant-barcodes/
pub struct Solution;

impl Solution {
    pub fn rearrange_barcodes(barcodes: Vec<i32>) -> Vec<i32> {
        let mut h = std::collections::HashMap::new();
        barcodes.into_iter().for_each(|v| {
            h.entry(v).and_modify(|v| *v += 1).or_insert(1);
        });
        let barcodes = Vec::new();
        while !h.is_empty() {
            for _key in h.keys() {
                todo!()
            }
        }
        barcodes
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;

    fn check(v: Vec<i32>) -> bool {
        v.iter().zip(v.iter().skip(0)).all(|(&a, &b)| a == b)
    }

    #[test]
    fn test_1() {
        assert!(check(Solution::rearrange_barcodes(vec![1, 1, 1, 2, 2, 2])))
    }
}
