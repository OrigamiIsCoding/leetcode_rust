#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/lru-cache/
use std::collections::HashMap;
use std::collections::VecDeque;

struct LRUItem<T> {
    value: T,
    count: usize,
}

impl<T> LRUItem<T> {
    fn new(value: T) -> Self {
        Self { value, count: 0 }
    }

    fn incr(&mut self) -> usize {
        self.count += 1;
        self.count
    }

    fn decr(&mut self) -> usize {
        self.count -= 1;
        self.count
    }
}

struct LRUCache {
    // gc
    queue: VecDeque<i32>,
    container: HashMap<i32, LRUItem<i32>>,
    capacity: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            queue: VecDeque::new(),
            container: HashMap::new(),
            capacity: capacity as usize,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        match self.container.get_mut(&key) {
            Some(item) => {
                Self::refresh(key, item, &mut self.queue);
                item.value
            }
            None => -1,
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        let item = match self.container.get_mut(&key) {
            Some(item) => {
                // modify
                item.value = value;
                item
            }
            None => {
                // check capacity
                if self.container.len() >= self.capacity {
                    self.remove_lru();
                }
                self.container.entry(key).or_insert(LRUItem::new(value))
            }
        };
        Self::refresh(key, item, &mut self.queue);
    }

    fn remove_lru(&mut self) {
        while let Some(key) = self.queue.pop_front() {
            if self.container.get_mut(&key).unwrap().decr() == 0 {
                self.container.remove(&key);
                break;
            }
        }
    }

    #[inline]
    fn refresh(key: i32, item: &mut LRUItem<i32>, queue: &mut VecDeque<i32>) {
        // like reference counting gc
        queue.push_back(key);
        item.incr();
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(1, cache.get(1));
        cache.put(3, 3);
        assert_eq!(-1, cache.get(2));
        cache.put(4, 4);
        assert_eq!(-1, cache.get(1));
        assert_eq!(3, cache.get(3));
        assert_eq!(4, cache.get(4));
    }

    #[test]
    fn test_2() {
        let mut cache = LRUCache::new(2);
        assert_eq!(-1, cache.get(2));
        cache.put(2, 6);
        assert_eq!(-1, cache.get(1));
        cache.put(1, 5);
        cache.put(1, 2);
        assert_eq!(2, cache.get(1));
        assert_eq!(6, cache.get(2));
    }
}
