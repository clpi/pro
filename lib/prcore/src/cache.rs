use std::collections::{BTreeSet, HashSet};

#[derive(Debug, Clone, Default)]
pub struct Cache<I: Copy + Clone + Into<usize>> {
    pub visited: HashSet<I>,
    visited_max_size: usize,
}
impl<I: Clone + Into<usize> + Copy> Cache<I> {
    pub fn new(vis_max_sz: I) -> Self {
        Self {
            visited: HashSet::with_capacity(vis_max_sz.into()),
            visited_max_size: vis_max_sz.into(),
        }
    }
    pub fn clear(&mut self) {
        self.visited.clear();
    }
    pub fn check(&mut self) {
        if self.visited_max_size > 0 &&
            self.visited.len() > self.visited_max_size {
            self.clear();
        }
    }
}
