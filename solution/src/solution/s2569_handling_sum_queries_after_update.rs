#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/handling-sum-queries-after-update/
pub struct Solution;

#[derive(Clone, Copy, Default)]
struct Node {
    l: usize,
    r: usize,
    count: i32,
    lazy_flip: bool,
}

impl Node {
    fn mid(self) -> usize {
        (self.l + self.r) / 2
    }
    fn flip(&mut self) {
        self.count = (self.r - self.l + 1) as i32 - self.count;
        self.lazy_flip = !self.lazy_flip;
    }
}

struct SegmentTree {
    nodes: Vec<Node>,
    len: usize,
}

macro_rules! left {
    ($u:ident) => {
        $u << 1
    };
}

macro_rules! right {
    ($u:ident) => {
        $u << 1 | 1
    };
}

macro_rules! node {
    ($self:ident, $u:expr) => {
        $self.nodes[$u]
    };
}

impl SegmentTree {
    fn new(nums: Vec<i32>) -> Self {
        let mut t = Self {
            nodes: vec![Default::default(); (nums.len() << 2) + 1],
            len: nums.len(),
        };
        t.build(1, 0, t.len - 1, &nums);
        t
    }

    fn build(&mut self, u: usize, l: usize, r: usize, nums: &Vec<i32>) {
        node!(self, u) = Node {
            l,
            r,
            ..Default::default()
        };
        if l >= r {
            node!(self, u).count = nums[l];
        } else {
            let mid = node!(self, u).mid();
            self.build(left!(u), l, mid, nums);
            self.build(right!(u), mid + 1, r, nums);
            self.push_up(u);
        }
    }

    fn push_up(&mut self, u: usize) {
        node!(self, u).count = node!(self, left!(u)).count + node!(self, right!(u)).count;
    }

    fn push_down(&mut self, u: usize) {
        if node!(self, u).lazy_flip {
            node!(self, left!(u)).flip();
            node!(self, right!(u)).flip();
            node!(self, u).lazy_flip = false;
        }
    }

    pub fn query(&mut self, l: usize, r: usize) -> i64 {
        self.query_impl(1, l, r)
    }

    fn query_impl(&mut self, u: usize, l: usize, r: usize) -> i64 {
        if node!(self, u).l >= l && node!(self, u).r <= r {
            node!(self, u).count as i64
        } else {
            self.push_down(u);
            let mut count = 0;

            let mid = node!(self, u).mid();
            if l <= mid {
                count += self.query_impl(left!(u), l, r);
            }
            if mid < r {
                count += self.query_impl(right!(u), l, r);
            }

            count
        }
    }

    pub fn modify(&mut self, l: usize, r: usize) {
        self.modify_impl(1, l, r);
    }

    fn modify_impl(&mut self, u: usize, l: usize, r: usize) {
        if node!(self, u).l >= l && node!(self, u).r <= r {
            node!(self, u).flip();
        } else {
            self.push_down(u);
            let mid = node!(self, u).mid();
            if l <= mid {
                self.modify_impl(left!(u), l, r);
            }
            if mid < r {
                self.modify_impl(right!(u), l, r);
            }
            self.push_up(u);
        }
    }
}

impl Solution {
    pub fn handle_query(nums1: Vec<i32>, nums2: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i64> {
        let mut sum = nums2.into_iter().map(|v| v as i64).sum::<i64>();
        let mut st = SegmentTree::new(nums1);
        let mut ans = Vec::new();
        for query in queries {
            let (l, r) = (query[1], query[2]);

            match query[0] {
                1 => st.modify(l as usize, r as usize),
                2 => sum += st.query(0_usize, st.len - 1) * l as i64,
                _ => ans.push(sum),
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::mat;

    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![3],
            Solution::handle_query(
                vec![1, 0, 1],
                vec![0, 0, 0],
                mat![[1, 1, 1], [2, 1, 0], [3, 0, 0]]
            )
        );
    }
}
