#![allow(dead_code)]
struct LUPrefix {
    union_find: Vec<usize>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LUPrefix {
    fn new(n: i32) -> Self {
        let mut union_find = vec![0; (n + 1) as usize];
        for (index, v) in union_find.iter_mut().enumerate() {
            *v = index;
        }
        Self { union_find }
    }

    fn upload(&mut self, video: i32) {
        self.union_find[(video - 1) as usize] = self.union_find[video as usize]
    }

    fn longest(&mut self) -> i32 {
        self.find(0) as i32
    }

    fn find(&mut self, x: usize) -> usize {
        if x != self.union_find[x] {
            self.union_find[x] = self.find(self.union_find[x])
        }
        self.union_find[x]
    }
}

/**
 * Your LUPrefix object will be instantiated and called as such:
 * let obj = LUPrefix::new(n);
 * obj.upload(video);
 * let ret_2: i32 = obj.longest();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut server = LUPrefix::new(4);
        server.upload(3);
        assert_eq!(0, server.longest());
        server.upload(1);
        assert_eq!(1, server.longest());
        server.upload(2);
        assert_eq!(3, server.longest());
    }
    
    #[test]
    fn test_2() {
        let mut server = LUPrefix::new(5);
        assert_eq!(0, server.longest());
        server.upload(1);
        assert_eq!(1, server.longest());
        server.upload(5);
        assert_eq!(1, server.longest());
    }
}
