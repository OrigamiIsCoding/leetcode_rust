#[derive(Debug)]
pub struct Node {
    pub id: usize,
    pub len: usize,
}

#[derive(Debug)]
pub struct UnionFind {
    values: Vec<Node>,
}

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        let values = (0..=n).map(|id| Node { id, len: 1 }).collect();
        UnionFind { values }
    }

    pub fn get(&mut self, id: usize) -> &Node {
        let id = self.find(id);
        &self.values[id]
    }

    pub fn get_id(&mut self, id: usize) -> usize {
        self.get(id).id
    }

    pub fn get_len(&mut self, id: usize) -> usize {
        self.get(id).len
    }

    pub fn union(&mut self, a: usize, b: usize) -> bool {
        let pa = self.find(a);
        let pb = self.find(b);
        if pa == pb {
            false
        } else {
            if self.values[pa].len < self.values[pb].len {
                self.values[pb].len += self.values[pa].len;
                self.values[pa].id = self.values[pb].id;
            } else {
                self.values[pa].len += self.values[pb].len;
                self.values[pb].id = self.values[pa].id;
            }
            true
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.values[x].id != x {
            self.values[x].id = self.find(self.values[x].id);
        }
        self.values[x].id
    }

    pub fn into_values(self) -> Vec<Node> {
        self.values
    }
}
