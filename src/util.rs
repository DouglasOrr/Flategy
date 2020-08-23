/// General-purpose utilities
use std::ops::Index;

/// A mapping from index to optional items, implemented using a Vec.
///
/// The mapping grows to hold the max index added.
///
#[derive(Clone, Default)]
pub struct VecMap<T: Clone> {
    items: Vec<Option<T>>,
}

impl<T: Clone> VecMap<T> {
    pub fn new() -> VecMap<T> {
        VecMap { items: Vec::new() }
    }

    pub fn insert(&mut self, index: usize, value: Option<T>) {
        if self.items.len() <= index {
            self.items.resize(index + 1, None);
        }
        self.items[index] = value;
    }
}

impl<T: Clone> Index<usize> for VecMap<T> {
    type Output = Option<T>;

    fn index(&self, index: usize) -> &Self::Output {
        if index < self.items.len() {
            &self.items[index]
        } else {
            &None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vecmap() {
        let mut map: VecMap<char> = VecMap::new();
        assert_eq!(None, map[0]);

        map.insert(1, Some('a'));
        assert_eq!(None, map[0]);
        assert_eq!(Some('a'), map[1]);
    }
}
