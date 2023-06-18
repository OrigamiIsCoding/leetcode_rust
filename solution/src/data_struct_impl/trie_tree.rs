#[derive(Clone)]
struct Item {
    count: usize,
    children: Vec<Option<Item>>,
}

impl Item {
    fn new(size: usize) -> Self {
        Self {
            count: 0,
            children: vec![None; size],
        }
    }
}

pub struct TrieTree {
    root: Item,
    children_size: usize,
}

impl TrieTree {
    pub fn new(children_size: usize) -> Self {
        Self {
            root: Item::new(children_size),
            children_size,
        }
    }

    pub fn insert<I>(&mut self, iter: I)
    where
        I: Iterator<Item = usize>,
    {
        let mut p = &mut self.root;
        for child in iter {
            if p.children[child].is_none() {
                p.children[child] = Some(Item::new(self.children_size));
            }
            p = p.children[child].as_mut().unwrap();
        }
        p.count += 1;
    }

    pub fn exists<I>(&self, iter: I) -> bool
    where
        I: Iterator<Item = usize>,
    {
        let mut p = &self.root;

        for child in iter {
            match &p.children[child] {
                None => return false,
                Some(child) => p = child,
            }
        }
        p.count > 0
    }
}
