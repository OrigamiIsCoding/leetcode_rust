#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/design-a-number-container-system/
use std::collections::{BTreeSet, HashMap};

#[derive(Default)]
struct NumberContainers {
    container: HashMap<i32, i32>,
    indexs: HashMap<i32, BTreeSet<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumberContainers {
    fn new() -> Self {
        Default::default()
    }

    fn change(&mut self, index: i32, number: i32) {
        if let Some(&old) = self.container.get(&index) {
            self.indexs.entry(old).and_modify(|v| {
                v.remove(&index);
            });
        }
        self.container.insert(index, number);
        self.indexs
            .entry(number)
            .or_insert_with(Default::default)
            .insert(index);
    }

    fn find(&self, number: i32) -> i32 {
        self.indexs
            .get(&number)
            .and_then(|index| index.iter().next()) // index.first()
            .copied()
            .unwrap_or(-1)
    }
}

/**
 * Your NumberContainers object will be instantiated and called as such:
 * let obj = NumberContainers::new();
 * obj.change(index, number);
 * let ret_2: i32 = obj.find(number);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut nc = NumberContainers::new();
        assert_eq!(-1, nc.find(10));
        nc.change(2, 10);
        nc.change(1, 10);
        nc.change(3, 10);
        nc.change(5, 10);
        assert_eq!(1, nc.find(10));
        nc.change(1, 20);
        assert_eq!(2, nc.find(10));
    }
}
