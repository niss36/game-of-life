use std;
use std::fmt::Display;

use wasm_bindgen::prelude::wasm_bindgen;

use crate::cell::Cell;
use crate::errors::ParseUniverseError;
use crate::grid::Grid;

#[wasm_bindgen]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Universe {
    pub(crate) cells: Grid<Cell>,
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

    fn count_live_neighbours(&self, coordinates: (usize, usize)) -> u8 {
        let mut count = 0;

        for column_offset in [self.cells.columns - 1, 0, 1] {
            for row_offset in [self.cells.rows - 1, 0, 1] {
                if column_offset == 0 && row_offset == 0 {
                    continue;
                }

                let neighbour_coordinates =
                    (coordinates.0 + column_offset, coordinates.1 + row_offset);

                count += *self.cells.get_wrapping(neighbour_coordinates) as u8;
            }
        }

        count
    }

    fn get_new_state(&self, coordinates: (usize, usize)) -> Cell {
        use Cell::*;

        let number_of_live_neighbours = self.count_live_neighbours(coordinates);

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

    const MOCK_UNIVERSE_WITH_PADDING: &str = "\
....
.##.
.##.
....";

    const MOCK_UNIVERSE_WITHOUT_PADDING: &str = "\
.#
#.";

    #[test]
    fn count_live_neighbours_works() {
        let universe: Universe = MOCK_UNIVERSE_WITH_PADDING.try_into().unwrap();

        assert_eq!(3, universe.count_live_neighbours((2, 1)));
    }

    #[test]
    fn count_live_neighbours_works_across_edge() {
        let universe: Universe = MOCK_UNIVERSE_WITHOUT_PADDING.try_into().unwrap();

        assert_eq!(4, universe.count_live_neighbours((0, 0)));
    }

    #[test]
    fn get_new_state_works() {
        let universe: Universe = MOCK_UNIVERSE_WITH_PADDING.try_into().unwrap();

        assert_eq!(Cell::Alive, universe.get_new_state((1, 1)));
    }
}
