use std::convert::From;
use std::iter::FromIterator;
use std::ops::{AddAssign, Sub};

pub struct PrefixSum<T> {
    container: Vec<T>,
}

impl<T> FromIterator<T> for PrefixSum<T>
where
    T: Copy + AddAssign + Default + Sub,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut container = vec![Default::default()];
        let mut acc = Default::default();
        for value in iter {
            acc += value;
            container.push(acc);
        }
        Self { container }
    }
}

impl<T> From<Vec<T>> for PrefixSum<T>
where
    T: Copy + AddAssign + Default + Sub,
{
    fn from(value: Vec<T>) -> Self {
        let mut container = vec![Default::default(); value.len() + 1];
        let mut acc = Default::default();
        for (i, value) in value.into_iter().enumerate() {
            acc += value;
            container[i + 1] = acc;
        }
        Self { container }
    }
}

impl<T> PrefixSum<T>
where
    T: Copy + AddAssign + Default + Sub<Output = T>,
{
    pub fn query(&self, left_bound: usize, right_bound: usize) -> T {
        self.container[right_bound] - self.container[left_bound - 1]
    }

    pub fn len(&self) -> usize {
        self.container.len() - 1
    }

    pub fn is_empty(&self) -> bool {
        self.container.len() == 1
    }
}
