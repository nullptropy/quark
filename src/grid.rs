use std::slice::{Iter, IterMut};

#[derive(Clone)]
pub struct Grid<T: Clone> {
    pub data: Vec<T>,
    cols: usize,
    rows: usize,
}

impl<T: Clone + std::fmt::Debug> std::fmt::Debug for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (i, _) in self.data.iter().enumerate().step_by(self.cols) {
            writeln!(f, "{:?}", &self.data[i..(i + self.cols)])?;
        }

        Ok(())
    }
}

impl<T: Clone> Grid<T> {
    pub fn new(cols: usize, rows: usize) -> Grid<T>
    where
        T: Default,
    {
        Self {
            data: vec![T::default(); cols * rows],
            cols,
            rows,
        }
    }

    pub fn iter(&self) -> Iter<T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.data.iter_mut()
    }

    pub unsafe fn get_unchecked(&self, col: usize, row: usize) -> &T {
        self.data.get_unchecked(col + self.cols * row)
    }

    pub unsafe fn get_unchecked_mut(&mut self, col: usize, row: usize) -> &mut T {
        self.data.get_unchecked_mut(col + self.cols * row)
    }

    pub fn get(&self, col: usize, row: usize) -> Option<&T> {
        if col < self.cols && row < self.rows {
            unsafe { Some(self.get_unchecked(col, row)) }
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, col: usize, row: usize) -> Option<&mut T> {
        if col < self.cols && row < self.rows {
            unsafe { Some(self.get_unchecked_mut(col, row)) }
        } else {
            None
        }
    }

    pub fn get_index(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.cols && y < self.rows {
            Some(x + self.cols * y)
        } else {
            None
        }
    }

    pub fn get_coords(&self, index: usize) -> Option<(usize, usize)> {
        if index < self.data.len() {
            Some((index % self.cols, index / self.cols))
        } else {
            None
        }
    }

    pub fn get_neighbours(&self, x: usize, y: usize) -> Vec<usize> {
        let (x, y) = (x as i32, y as i32);
        let mut result = Vec::new();

        for dx in -1..1 {
            for dy in -1..1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                if let Some(index) = self.get_index((x + dx) as usize, (y + dy) as usize) {
                    result.push(index);
                }
            }
        }

        result
    }
}
