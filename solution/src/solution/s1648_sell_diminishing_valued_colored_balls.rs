#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/sell-diminishing-valued-colored-balls/
pub struct Solution;

impl Solution {
    pub fn max_profit(mut inventory: Vec<i32>, orders: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        // 排序从最大的开始
        inventory.sort();
        let mut result = 0;
        let mut orders = orders as i64;
        // peekable 用于查看下一个
        let mut iter = inventory.into_iter().rev().peekable();
        // 目前有多少个相同数目的
        let mut multiple = 1;

        // 等差数列求和
        let sum = |l, r| ((l + r) as i64 * (r - l + 1) as i64 / 2) % MOD;

        while let Some(count) = iter.next() {
            let count = count as i64;
            let d = i64::min(
                orders,
                (count - iter.peek().copied().unwrap_or(0) as i64) * multiple,
            );

            orders -= d;

            let (total, remain) = (d / multiple, d % multiple);

            // 整数个直接加
            if total != 0 {
                result += multiple * sum(count - total + 1, count)
            }
            // 剩下的再加
            result += remain * (count - total);

            result %= MOD;
            if orders == 0 {
                break;
            }
            multiple += 1;
        }

        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(14, Solution::max_profit(vec![2, 5], 4));
    }

    #[test]
    fn test_2() {
        assert_eq!(19, Solution::max_profit(vec![3, 5], 6));
    }

    #[test]
    fn test_3() {
        assert_eq!(110, Solution::max_profit(vec![2, 8, 4, 10, 6], 20));
    }

    #[test]
    fn test_4() {
        assert_eq!(21, Solution::max_profit(vec![1000000000], 1000000000));
    }

    #[test]
    fn test_5() {
        assert_eq!(
            997286992,
            Solution::max_profit(
                vec![
                    565259708, 715164401, 716563713, 958255469, 844600740, 823949511, 180479359,
                    287829385, 164248818, 73361150, 230686692, 322986846, 598720034, 338241127,
                    748922260, 181241085, 833659853, 509571179, 250093451, 690995620, 703292727,
                    595636202
                ],
                650114768
            )
        );
    }

    #[test]
    fn test_6() {
        assert_eq!(
            37,
            Solution::max_profit(vec![1000000000, 1000000000, 1000000000], 1000000000)
        );
    }
}
