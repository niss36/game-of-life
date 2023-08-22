use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::CanvasRenderingContext2d;

use crate::{cell::Cell, universe::Universe};

#[wasm_bindgen]
impl Universe {
    pub fn render_to_canvas(
        &self,
        context: CanvasRenderingContext2d,
        cell_size: usize,
        grid_colour: &JsValue,
        dead_colour: &JsValue,
        alive_colour: &JsValue,
    ) {
        let rows = self.cells.rows;
        let columns = self.cells.columns;

        context.set_fill_style(grid_colour);
        context.fill_rect(
            0.,
            0.,
            ((cell_size + 1) * columns + 1) as f64,
            ((cell_size + 1) * rows + 1) as f64,
        );

        for column in 0..columns {
            for row in 0..rows {
                let cell = self.cells.get((column, row)).unwrap();

                let fill_style = match cell {
                    Cell::Dead => dead_colour,
                    Cell::Alive => alive_colour,
                };
                context.set_fill_style(fill_style);

                context.fill_rect(
                    (column * (cell_size + 1) + 1) as f64,
                    (row * (cell_size + 1) + 1) as f64,
                    cell_size as f64,
                    cell_size as f64,
                )
            }
        }
    }
}
