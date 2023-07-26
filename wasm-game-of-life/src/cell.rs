use std::fmt::Display;

use crate::errors::ParseUniverseError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    pub fn new_random() -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            (js_sys::Math::random() > 0.5).into()
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            rand::random::<bool>().into()
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Cell::*;

        match self {
            Dead => write!(f, "."),
            Alive => write!(f, "#"),
        }
    }
}

impl TryFrom<char> for Cell {
    type Error = ParseUniverseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Cell::*;

        match value {
            '.' => Ok(Dead),
            '#' => Ok(Alive),
            c => Err(ParseUniverseError::InvalidCell(c)),
        }
    }
}

impl From<bool> for Cell {
    fn from(value: bool) -> Self {
        use Cell::*;

        if value {
            Alive
        } else {
            Dead
        }
    }
}
