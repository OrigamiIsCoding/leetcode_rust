use std::collections::HashMap;
struct MapSum {
    h: HashMap<String, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MapSum {
    fn new() -> Self {
        Self {
            h: HashMap::new(),
        }
    }

    fn insert(&mut self, key: String, val: i32) {
        self.h.insert(key, val);
    }

    fn sum(&self, prefix: String) -> i32 {
        let mut sum = 0;
        for (key, val) in self.h.iter() {
            if key.starts_with(&prefix) {
                sum += *val;
            } 
        }
        sum
    }
}

/**
 * Your MapSum object will be instantiated and called as such:
 * let obj = MapSum::new();
 * obj.insert(key, val);
 * let ret_2: i32 = obj.sum(prefix);
 */
#[cfg(test)]
mod tests {
    use super::*;

    /// ["MapSum", "insert", "sum", "insert", "sum"]
    /// [[], ["apple", 3], ["ap"], ["app", 2], ["ap"]]
    #[test]
    fn test_1() {
        let mut obj = MapSum::new();
        
        obj.insert("apple".into(), 3);

        assert_eq!(3, obj.sum("ap".into()));

        obj.insert("app".into(), 2);

        assert_eq!(5, obj.sum("ap".into()));
    }
}
