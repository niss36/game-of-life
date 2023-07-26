#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Grid<T> {
    pub(crate) items: Vec<T>,
    pub(crate) columns: usize,
    pub(crate) rows: usize,
}

fn index_to_coordinates(columns: usize, index: usize) -> (usize, usize) {
    (index % columns, index / columns)
}

fn coordinates_to_index(columns: usize, coordinates: (usize, usize)) -> usize {
    coordinates.0 + columns * coordinates.1
}

impl<T> Grid<T> {
    pub fn new<F>(columns: usize, rows: usize, initialiser: F) -> Self
    where
        F: Fn((usize, usize)) -> T,
    {
        let items = (0..(columns * rows))
            .map(|index| initialiser(index_to_coordinates(columns, index)))
            .collect();

        Self {
            items,
            columns,
            rows,
        }
    }

    pub fn get(&self, coordinates: (usize, usize)) -> Option<&T> {
        if coordinates.0 >= self.columns || coordinates.1 >= self.rows {
            return None;
        }

        self.items
            .get(coordinates_to_index(self.columns, coordinates))
    }

    pub fn get_mut(&mut self, coordinates: (usize, usize)) -> Option<&mut T> {
        if coordinates.0 >= self.columns || coordinates.1 >= self.rows {
            return None;
        }

        self.items
            .get_mut(coordinates_to_index(self.columns, coordinates))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_to_coordinates_works_when_index_less_than_columns() {
        assert_eq!(index_to_coordinates(5, 1), (1, 0));
    }

    #[test]
    fn test_index_to_coordinates_works_when_index_greater_than_columns() {
        assert_eq!(index_to_coordinates(5, 11), (1, 2));
    }

    #[test]
    fn test_coordinates_to_index_works() {
        assert_eq!(coordinates_to_index(5, (1, 2)), 11);
    }

    #[test]
    fn test_grid_get_works() {
        let grid = Grid::new(5, 5, |_| ());

        assert_eq!(grid.get((2, 3)), Some(&()));
    }

    #[test]
    fn test_grid_get_returns_none_when_out_of_bounds() {
        let grid = Grid::new(5, 5, |_| ());

        assert_eq!(grid.get((5, 3)), None);
    }
}
