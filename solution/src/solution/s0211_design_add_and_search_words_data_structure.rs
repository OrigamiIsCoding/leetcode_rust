#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/design-add-and-search-words-data-structure/
pub struct Solution;

#[derive(Clone)]
struct Item {
    end: bool,
    children: Vec<Option<Item>>,
}

impl Item {
    fn new() -> Self {
        Self {
            end: false,
            children: vec![None; 26],
        }
    }
}

struct WordDictionary {
    root: Item,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    fn new() -> Self {
        Self { root: Item::new() }
    }

    fn add_word(&mut self, word: String) {
        let mut p = &mut self.root;
        for child in word.bytes().map(|b| (b - b'a') as usize) {
            if p.children[child].is_none() {
                p.children[child] = Some(Item::new());
            }
            p = p.children[child].as_mut().unwrap();
        }
        p.end = true;
    }

    fn search(&self, word: String) -> bool {
        fn dfs(u: usize, root: &Item, word: &[u8]) -> bool {
            if u >= word.len() {
                root.end
            } else {
                match word[u] {
                    b'.' => (0..26).any(|p| match &root.children[p] {
                        Some(child) => dfs(u + 1, child, word),
                        None => false,
                    }),
                    _ => match &root.children[(word[u] - b'a') as usize] {
                        Some(child) => dfs(u + 1, child, word),
                        None => false,
                    },
                }
            }
        }

        dfs(0, &self.root, word.as_bytes())
    }
}

/**
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut word_dictionary = WordDictionary::new();
        word_dictionary.add_word("bad".into());
        word_dictionary.add_word("dad".into());
        word_dictionary.add_word("mad".into());

        assert!(!word_dictionary.search("pad".into()));
        assert!(word_dictionary.search("bad".into()));
        assert!(word_dictionary.search(".ad".into()));
        assert!(word_dictionary.search("b..".into()));
    }
}
