#![allow(dead_code)]
pub struct Solution;

impl Solution {
    fn f(word: String) -> i32 {
        let mut min_ch = u8::MAX;
        let mut cnt = 0;
        for ch in word.bytes() {
            match ch.cmp(&min_ch) {
                std::cmp::Ordering::Less => {
                    cnt = 1;
                    min_ch = ch;
                }
                std::cmp::Ordering::Equal => cnt += 1,
                _ => (),
            }
        }
        cnt
    }

    pub fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
        let words_count: Vec<_> = words.into_iter().map(Self::f).collect();
        queries
            .into_iter()
            .map(|query| {
                let cnt = Self::f(query);

                words_count.iter().filter(|&&v| cnt < v).count() as i32
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::intovec;

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![1],
            Solution::num_smaller_by_frequency(intovec!["cbd"], intovec!["zaaaz"])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![1, 2],
            Solution::num_smaller_by_frequency(
                intovec!["bbb", "cc"],
                intovec!["a", "aa", "aaa", "aaaa"]
            )
        );
    }
}
