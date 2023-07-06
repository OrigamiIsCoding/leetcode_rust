#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/find-consecutive-integers-from-a-data-stream/
struct DataStream {
    count: i32,
    k: i32,
    value: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl DataStream {
    fn new(value: i32, k: i32) -> Self {
        Self { count: 0, k, value }
    }

    fn consec(&mut self, num: i32) -> bool {
        if num != self.value {
            self.count = 0;
        } else {
            self.count += 1;
        }
        self.count >= self.k
    }
}

/**
 * Your DataStream object will be instantiated and called as such:
 * let obj = DataStream::new(value, k);
 * let ret_1: bool = obj.consec(num);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut ds = DataStream::new(4, 3);
        assert!(!ds.consec(4));
        assert!(!ds.consec(4));
        assert!(ds.consec(4));
        assert!(!ds.consec(3));
    }
}
