#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/can-make-palindrome-from-substring/
pub struct Solution;
use crate::data_struct_impl::PrefixSum;

#[derive(Default, Clone, Copy, Debug)]
struct Item {
    count: [i32; 26],
}

impl std::ops::AddAssign for Item {
    fn add_assign(&mut self, rhs: Self) {
        for (i, c) in rhs.count.into_iter().enumerate() {
            self.count[i] += c;
        }
    }
}

impl std::ops::Sub for Item {
    type Output = Item;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut output = Item::default();
        for (index, (left, right)) in self
            .count
            .into_iter()
            .zip(rhs.count.into_iter())
            .enumerate()
        {
            output.count[index] = left - right;
        }
        output
    }
}

impl From<u8> for Item {
    fn from(value: u8) -> Self {
        let mut count = [0; 26];
        count[(value - 'a' as u8) as usize] += 1;
        Self { count }
    }
}

impl Solution {
    pub fn can_make_pali_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let ps: PrefixSum<Item> = PrefixSum::from(s.bytes().map(|b| b.into()).collect::<Vec<_>>());

        queries
            .into_iter()
            .map(|query| {
                let (left, right, k) = (query[0], query[1], query[2]);
                let cnt = right - left + 1;

                let result = ps.query((left + 1) as usize, (right + 1) as usize);

                let odd_cnt: i32 = result.count.iter().map(|&v| v % 2).sum();

                if cnt % 2 == 0 {
                    k >= odd_cnt / 2
                } else {
                    k >= (odd_cnt - 1) / 2
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::mat;

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![true, false, false, true, true],
            Solution::can_make_pali_queries(
                "abcda".into(),
                mat![[3, 3, 0], [1, 2, 0], [0, 3, 1], [0, 3, 2], [0, 4, 1]]
            )
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![false, true],
            Solution::can_make_pali_queries("lyb".into(), mat![[0, 1, 0], [2, 2, 1]])
        );
    }
}
