#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/rings-and-rods/
pub struct Solution;

impl Solution {
    pub fn count_points(rings: String) -> i32 {
        let mut states = [0; 10];
        let mut rings = rings.into_bytes().into_iter();
        while let Some(color) = rings.next() {
            let idx = rings.next().unwrap() - b'0';
            states[idx as usize] |= match color {
                b'R' => 0b1,
                b'G' => 0b10,
                _ => 0b100,
            };
        }
        states.into_iter().filter(|&v| v == 0b111).count() as i32
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::count_points("B0B6G0R6R0R6G9".into()));
    }

    #[test]
    fn test_2() {
        assert_eq!(1, Solution::count_points("B0R0G0R9R0B0G0".into()));
    }

    #[test]
    fn test_3() {
        assert_eq!(0, Solution::count_points("G4".into()));
    }
}
