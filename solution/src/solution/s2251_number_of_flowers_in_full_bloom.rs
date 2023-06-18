#![allow(dead_code)]

pub struct Solution;

impl Solution {
    pub fn full_bloom_flowers(flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32> {
        // 离散化
        let mut alls: Vec<_> = flowers
            .iter()
            .flatten()
            .chain(people.iter())
            .copied()
            .collect();
        alls.sort();
        alls.dedup();

        let find = |x| alls.binary_search(&x).unwrap();
        // 差分
        let mut s = vec![0; alls.len() + 1];

        for flower in flowers {
            let (l, r) = (flower[0], flower[1]);
            s[find(l)] += 1;
            s[find(r) + 1] -= 1;
        }

        for i in 1..s.len() {
            s[i] += s[i - 1];
        }
        people.into_iter().map(|t| s[find(t)]).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::mat;

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![1, 2, 2, 2],
            Solution::full_bloom_flowers(mat![[1, 6], [3, 7], [9, 12], [4, 13]], vec![2, 3, 7, 11])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![2, 2, 1],
            Solution::full_bloom_flowers(mat![[1, 10], [3, 3]], vec![3, 3, 2])
        );
    }
}
