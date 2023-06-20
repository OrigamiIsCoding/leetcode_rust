use std::ops::AddAssign;

/// 树状数组
pub struct FenwickTree<T>
where
    T: AddAssign + Copy + Default,
{
    values: Vec<T>,
    size: usize,
}

impl<T> FenwickTree<T>
where
    T: AddAssign + Copy + Default,
{
    #[inline]
    fn lowbit(x: usize) -> usize {
        x & x.wrapping_neg()
    }

    pub fn new(size: usize) -> Self {
        Self {
            values: vec![Default::default(); size + 1],
            size,
        }
    }

    pub fn add(&mut self, mut index: usize, d: T) {
        while index <= self.size {
            self.values[index] += d;
            index += Self::lowbit(index);
        }
    }

    pub fn query(&self, mut rbound: usize) -> T {
        let mut sum = Default::default();
        while rbound > 0 {
            sum += self.values[rbound];
            rbound -= Self::lowbit(rbound);
        }
        sum
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}
