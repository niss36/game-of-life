use wasm_bindgen::JsValue;

#[derive(Debug, Clone, Copy)]
pub enum ParseUniverseError {
    InvalidCell(char),
    MismatchedRowLengths(usize, usize),
}

impl From<ParseUniverseError> for JsValue {
    fn from(value: ParseUniverseError) -> Self {
        use ParseUniverseError::*;

        match value {
            InvalidCell(c) => {
                format!("Invalid cell value '{c}', expected one of '.' or '#'").into()
            }
            MismatchedRowLengths(expected, actual) => {
                format!("Mismatched row lengths, expected {expected} but got {actual}").into()
            }
        }
    }
}
