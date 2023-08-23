use itertools::{iproduct, Itertools};
use js_sys::Uint32Array;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::{
    WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlTexture, WebGlUniformLocation,
};

use crate::universe::Universe;

#[wasm_bindgen]
pub struct WebGlProgramInfo {
    vertex_count: i32,
    cells_uniform_location: WebGlUniformLocation,
    cells_texture: WebGlTexture,
}

#[wasm_bindgen]
impl Universe {
    pub fn setup_web_gl(
        &self,
        context: &WebGl2RenderingContext,
        cell_size: usize,
    ) -> Result<WebGlProgramInfo, JsValue> {
        let vert_shader = compile_shader(
            context,
            WebGl2RenderingContext::VERTEX_SHADER,
            include_str!("vertex_shader.glsl"),
        )?;

        let frag_shader = compile_shader(
            context,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            include_str!("fragment_shader.glsl"),
        )?;

        let program = link_program(context, &vert_shader, &frag_shader)?;
        context.use_program(Some(&program));

        let position_attribute_location = context.get_attrib_location(&program, "position");
        let coordinates_attribute_location = context.get_attrib_location(&program, "coordinates");

        let window_size_uniform_location = context
            .get_uniform_location(&program, "window_size")
            .ok_or("Failed to get uniform location for `window_size`")?;

        let universe_size_uniform_location = context
            .get_uniform_location(&program, "universe_size")
            .ok_or("Failed to get uniform location for `universe_size`")?;

        let cells_uniform_location = context
            .get_uniform_location(&program, "cells")
            .ok_or("Failed to get uniform location for `cells`")?;

        let columns = self.cells.columns;
        let rows = self.cells.rows;

        context.uniform2f(
            Some(&window_size_uniform_location),
            ((cell_size + 1) * columns + 1) as f32,
            ((cell_size + 1) * rows + 1) as f32,
        );

        context.uniform2f(
            Some(&universe_size_uniform_location),
            (columns - 1) as f32,
            (rows - 1) as f32,
        );

        let vertices = iproduct!(0..columns, 0..rows)
            .flat_map(|(column, row)| -> [usize; 24] {
                let x_start = column * (cell_size + 1) + 1;
                let y_start = row * (cell_size + 1) + 1;

                let x_end = x_start + cell_size;
                let y_end = y_start + cell_size;

                // Flip Y axis because it points up in WebGL
                let row = rows - row - 1;

                [
                    x_start, y_end, column, row, x_start, y_start, column, row, x_end, y_start,
                    column, row, x_end, y_start, column, row, x_end, y_end, column, row, x_start,
                    y_end, column, row,
                ]
            })
            .map(|value| value as u32)
            .collect_vec();

        let buffer = context.create_buffer().ok_or("Failed to create buffer")?;
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

        // Note that `Uint32Array::view` is somewhat dangerous (hence the
        // `unsafe`!). This is creating a raw view into our module's
        // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
        // (aka do a memory allocation in Rust) it'll cause the buffer to change,
        // causing the `Uint32Array` to be invalid.
        //
        // As a result, after `Uint32Array::view` we have to be very careful not to
        // do any memory allocations before it's dropped.
        unsafe {
            let vertex_buffer_view = Uint32Array::view(&vertices);

            context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &vertex_buffer_view,
                WebGl2RenderingContext::DYNAMIC_DRAW,
            );
        }

        let vao = context
            .create_vertex_array()
            .ok_or("Could not create vertex array object")?;
        context.bind_vertex_array(Some(&vao));

        context.vertex_attrib_pointer_with_i32(
            position_attribute_location as u32,
            2,
            WebGl2RenderingContext::UNSIGNED_INT,
            false,
            16,
            0,
        );
        context.enable_vertex_attrib_array(position_attribute_location as u32);

        context.vertex_attrib_pointer_with_i32(
            coordinates_attribute_location as u32,
            2,
            WebGl2RenderingContext::UNSIGNED_INT,
            false,
            16,
            8,
        );
        context.enable_vertex_attrib_array(coordinates_attribute_location as u32);

        context.bind_vertex_array(Some(&vao));

        let cells_texture = context.create_texture().ok_or("Failed to create texture")?;
        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&cells_texture));
        context.pixel_storei(WebGl2RenderingContext::UNPACK_ALIGNMENT, 1);
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_S,
            WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
        );
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_T,
            WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
        );
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MIN_FILTER,
            WebGl2RenderingContext::NEAREST as i32,
        );
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MAG_FILTER,
            WebGl2RenderingContext::NEAREST as i32,
        );

        Ok(WebGlProgramInfo {
            vertex_count: (vertices.len() / 4) as i32,
            cells_uniform_location,
            cells_texture,
        })
    }

    pub fn render_to_web_gl(
        &self,
        context: &WebGl2RenderingContext,
        program_info: &WebGlProgramInfo,
    ) -> Result<(), JsValue> {
        let cells = self
            .cells
            .items
            .iter()
            .map(|cell| *cell as u8)
            .collect_vec();

        context.bind_texture(
            WebGl2RenderingContext::TEXTURE_2D,
            Some(&program_info.cells_texture),
        );
        context
            .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_u8_array_and_src_offset(
                WebGl2RenderingContext::TEXTURE_2D,
                0,
                WebGl2RenderingContext::ALPHA as i32,
                self.cells.columns as i32,
                self.cells.rows as i32,
                0,
                WebGl2RenderingContext::ALPHA,
                WebGl2RenderingContext::UNSIGNED_BYTE,
                &cells,
                0,
            )?;
        context.active_texture(WebGl2RenderingContext::TEXTURE0);

        context.uniform1i(Some(&program_info.cells_uniform_location), 0);

        context.clear_color(0.75, 0.75, 0.75, 1.0);
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        context.draw_arrays(
            WebGl2RenderingContext::TRIANGLES,
            0,
            program_info.vertex_count,
        );

        Ok(())
    }
}

fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

fn link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}
