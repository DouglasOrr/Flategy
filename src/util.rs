/// General-purpose utilities
use std::ops::{Index, IndexMut};

// VecMap

/// A mapping from index to optional items, implemented using a Vec.
///
/// The mapping grows to hold the max index added.
///
#[derive(Clone, Default)]
pub struct VecMap<T> {
    items: Vec<Option<T>>,
}

impl<T: Clone> VecMap<T> {
    pub fn new() -> Self {
        VecMap { items: Vec::new() }
    }

    pub fn insert(&mut self, index: usize, value: Option<T>) {
        if self.items.len() <= index {
            self.items.resize(index + 1, None);
        }
        self.items[index] = value;
    }
}

impl<T> Index<usize> for VecMap<T> {
    type Output = Option<T>;

    fn index(&self, index: usize) -> &Self::Output {
        if index < self.items.len() {
            &self.items[index]
        } else {
            &None
        }
    }
}

// GridIndex

/// A type that can be used as a multidimensional index into a Grid
pub trait GridIndex: Copy + std::fmt::Debug {
    fn size(self) -> usize;
    fn in_bounds(self, index: Self) -> bool;
    fn offset(self, index: Self) -> usize;
}

impl GridIndex for (usize, usize) {
    fn size(self) -> usize {
        self.0 * self.1
    }
    fn in_bounds(self, index: Self) -> bool {
        index.0 < self.0 && index.1 < self.1
    }
    fn offset(self, index: Self) -> usize {
        index.0 + index.1 * self.0
    }
}

impl GridIndex for (usize, usize, usize) {
    fn size(self) -> usize {
        self.0 * self.1 * self.2
    }
    fn in_bounds(self, index: Self) -> bool {
        index.0 < self.0 && index.1 < self.1 && index.2 < self.2
    }
    fn offset(self, index: Self) -> usize {
        index.0 + index.1 * self.0 + index.2 * self.0 * self.1
    }
}

// Grid

/// An N-Dimensional grid of items, stored flattened in a Vec
pub struct Grid<I, T> {
    shape: I,
    data: Vec<T>,
}

impl<I, T> Grid<I, T>
where
    I: GridIndex,
    T: Clone,
{
    pub fn new(shape: I, item: &T) -> Self {
        let data = (0..shape.size()).map(|_| item.clone()).collect();
        Grid {
            shape: shape,
            data: data,
        }
    }
}

impl<I, T> Grid<I, T>
where
    I: GridIndex,
{
    pub fn shape(&self) -> I {
        self.shape
    }
    pub fn data(&self) -> &Vec<T> {
        &self.data
    }
    pub fn len(&self) -> usize {
        self.shape.size()
    }
    fn checked_offset(&self, index: I) -> usize {
        if !self.shape.in_bounds(index) {
            panic!("index {:?} out of bounds for shape {:?}", index, self.shape);
        }
        self.shape.offset(index)
    }
}

impl<I, T> Index<I> for Grid<I, T>
where
    I: GridIndex,
{
    type Output = T;

    fn index(&self, index: I) -> &Self::Output {
        &self.data[self.checked_offset(index)]
    }
}

impl<I, T> IndexMut<I> for Grid<I, T>
where
    I: GridIndex,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        let offset = self.checked_offset(index);
        &mut self.data[offset]
    }
}

// Tests

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

    #[test]
    fn test_grid_2d() {
        let mut grid = Grid::new((2, 3), &'a');
        assert_eq!(6, grid.len());
        assert_eq!((2, 3), grid.shape());
        assert_eq!('a', grid[(1, 2)]);

        grid[(1, 2)] = 'b';
        assert_eq!('b', grid[(1, 2)]);
        assert_eq!('a', grid[(1, 1)]);
    }

    #[test]
    fn test_grid_3d() {
        let mut grid = Grid::new((2, 3, 4), &'a');
        assert_eq!(24, grid.len());
        assert_eq!((2, 3, 4), grid.shape());
        assert_eq!('a', grid[(1, 2, 3)]);

        grid[(1, 2, 3)] = 'b';
        assert_eq!('b', grid[(1, 2, 3)]);
        assert_eq!('a', grid[(0, 2, 3)]);
    }

    #[test]
    fn test_grid_no_overlap() {
        // Make sure each index maps to a unique location
        let mut grid = Grid::new((5, 6, 7), &false);
        for i in 0..(grid.shape.0) {
            for j in 0..(grid.shape.1) {
                for k in 0..(grid.shape.2) {
                    assert!(!grid[(i, j, k)]);
                    grid[(i, j, k)] = true;
                }
            }
        }
    }
}
