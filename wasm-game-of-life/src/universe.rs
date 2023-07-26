use std;
use std::fmt::Display;

use wasm_bindgen::prelude::*;

use crate::cell::Cell;
use crate::errors::ParseUniverseError;
use crate::grid::Grid;

#[derive(Debug, Clone, Copy)]
enum Offset {
    TopLeft,
    Top,
    TopRight,
    Left,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}

impl Offset {
    const OFFSETS: [Self; 8] = [
        Self::TopLeft,
        Self::Top,
        Self::TopRight,
        Self::Left,
        Self::Right,
        Self::BottomLeft,
        Self::Bottom,
        Self::BottomRight,
    ];

    fn get_coordinates(self, base_coordinates: (usize, usize)) -> Option<(usize, usize)> {
        use Offset::*;

        let (x, y) = base_coordinates;

        match self {
            TopLeft => Some((x.checked_sub(1)?, y.checked_sub(1)?)),
            Top => Some((x, y.checked_sub(1)?)),
            TopRight => Some((x.checked_add(1)?, y.checked_sub(1)?)),
            Left => Some((x.checked_sub(1)?, y)),
            Right => Some((x.checked_add(1)?, y)),
            BottomLeft => Some((x.checked_sub(1)?, y.checked_add(1)?)),
            Bottom => Some((x, y.checked_add(1)?)),
            BottomRight => Some((x.checked_add(1)?, y.checked_add(1)?)),
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Universe {
    cells: Grid<Cell>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new_empty(columns: usize, rows: usize) -> Self {
        Self {
            cells: Grid::new(columns, rows, |_| Cell::Dead),
        }
    }

    pub fn new_random(columns: usize, rows: usize) -> Self {
        Self {
            cells: Grid::new(columns, rows, |_| Cell::new_random()),
        }
    }

    fn get_number_of_live_neighbours(&self, coordinates: (usize, usize)) -> u8 {
        let neighbour_states = Offset::OFFSETS
            .into_iter()
            .filter_map(|offset| offset.get_coordinates(coordinates))
            .filter_map(|neighbour_coordinates| self.cells.get(neighbour_coordinates));

        neighbour_states.map(|&cell| cell as u8).sum()
    }

    fn get_new_state(&self, coordinates: (usize, usize)) -> Cell {
        use Cell::*;

        let number_of_live_neighbours = self.get_number_of_live_neighbours(coordinates);

        let current_state = self.cells.get(coordinates).unwrap_or(&Dead);

        match (current_state, number_of_live_neighbours) {
            (Alive, 2) => Alive,
            (Alive, 3) => Alive,
            (Dead, 3) => Alive,
            _ => Dead,
        }
    }

    pub fn step(&self) -> Self {
        Self {
            cells: Grid::new(self.cells.columns, self.cells.rows, |coordinates| {
                self.get_new_state(coordinates)
            }),
        }
    }

    pub fn parse(value: &str) -> Result<Universe, ParseUniverseError> {
        value.try_into()
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl TryFrom<&str> for Universe {
    type Error = ParseUniverseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut cells = vec![];

        let mut columns: Option<usize> = None;

        for row in value.trim_end().split('\n') {
            if let Some(columns) = columns {
                if row.len() != columns {
                    return Err(ParseUniverseError::MismatchedRowLengths(columns, row.len()));
                }
            } else {
                columns = Some(row.len());
            }

            cells.push(row.chars().map(Cell::try_from));
        }

        let cells = cells.into_iter().flatten().collect::<Result<Vec<_>, _>>()?;

        let columns = columns.unwrap_or_default();
        let rows = if columns == 0 {
            0
        } else {
            cells.len() / columns
        };

        Ok(Self {
            cells: Grid {
                items: cells,
                columns,
                rows,
            },
        })
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.cells.columns == 0 {
            return Ok(());
        }

        for row in self.cells.items.chunks_exact(self.cells.columns) {
            for cell in row {
                write!(f, "{cell}")?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_live_neighbours() {
        let universe: Universe = "##\n##".try_into().unwrap();

        assert_eq!(3, universe.get_number_of_live_neighbours((1, 0)));
    }

    #[test]
    fn test_get_new_state() {
        let universe: Universe = "##\n##".try_into().unwrap();

        assert_eq!(Cell::Alive, universe.get_new_state((0, 0)));
    }
}
