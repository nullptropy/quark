use crate::grid::Grid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Particle {
    Empty,
    Sand,
    Wall,
    Water,
}

impl Default for Particle {
    fn default() -> Self {
        Particle::Empty
    }
}

impl Particle {
    pub fn color(&self) -> Option<(u8, u8, u8)> {
        match self {
            Self::Empty => None,
            Self::Sand => Some((253, 249, 0)),
            Self::Wall => Some((130, 130, 130)),
            Self::Water => Some((0, 121, 241)),
        }
    }

    fn sand_update(&self, x: i32, y: i32, grid: &mut Grid<Self>) {
        let self_index = grid.get_index(x as usize, y as usize).unwrap();

        for (dx, dy) in [(0, 1), (-1, 1), (1, 1)] {
            let (nx, ny) = (x + dx, y + dy);

            if let Some(index) = grid.get_index(nx as usize, ny as usize) {
                match grid.data[index] {
                    Self::Empty => {
                        grid.data[index] = Self::Sand;
                        grid.data[self_index] = Self::Empty;
                        return;
                    }
                    Self::Water => {
                        grid.data.swap(index, self_index);
                        return;
                    }
                    _ => (),
                }
            }
        }
    }

    fn water_update(&self, x: i32, y: i32, grid: &mut Grid<Self>) {
        let self_index = grid.get_index(x as usize, y as usize).unwrap();

        for (dx, dy) in [(0, 1), (-1, 1), (1, 1), (-1, 0), (1, 0)] {
            let (nx, ny) = (x + dx, y + dy);

            if let Some(index) = grid.get_index(nx as usize, ny as usize) {
                match grid.data[index] {
                    Self::Empty => {
                        grid.data[index] = Self::Water;
                        grid.data[self_index] = Self::Empty;
                        return;
                    }
                    _ => (),
                }
            }
        }
    }

    pub fn update(&self, x: i32, y: i32, grid: &mut Grid<Self>) {
        match self {
            Self::Empty => (),
            Self::Wall => (),
            Self::Sand => self.sand_update(x, y, grid),
            Self::Water => self.water_update(x, y, grid),
        }
    }
}
