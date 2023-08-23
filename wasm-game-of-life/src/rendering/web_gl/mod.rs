use itertools::{iproduct, Itertools};
use js_sys::Uint32Array;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::{console, WebGl2RenderingContext, WebGlProgram, WebGlShader};

use crate::universe::Universe;

#[wasm_bindgen]
impl Universe {
    pub fn render_to_web_gl(
        &self,
        context: WebGl2RenderingContext,
        cell_size: usize,
    ) -> Result<(), JsValue> {
        console::time_with_label("Compile shaders");
        let vert_shader = compile_shader(
            &context,
            WebGl2RenderingContext::VERTEX_SHADER,
            include_str!("vertex_shader.glsl"),
        )?;

        let frag_shader = compile_shader(
            &context,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            include_str!("fragment_shader.glsl"),
        )?;

        let program = link_program(&context, &vert_shader, &frag_shader)?;
        context.use_program(Some(&program));
        console::time_end_with_label("Compile shaders");

        let position_attribute_location = context.get_attrib_location(&program, "position");
        let cell_attribute_location = context.get_attrib_location(&program, "cell");

        let universe_size_uniform_location = context
            .get_uniform_location(&program, "universe_size")
            .ok_or("Failed to get uniform location for `universe_size`")?;

        let columns = self.cells.columns;
        let rows = self.cells.rows;

        context.uniform2f(
            Some(&universe_size_uniform_location),
            ((cell_size + 1) * columns + 1) as f32,
            ((cell_size + 1) * rows + 1) as f32,
        );

        console::time_with_label("Compute vertices");
        let vertices = iproduct!(0..columns, 0..rows)
            .flat_map(|(column, row)| -> [usize; 18] {
                let cell = self.cells.get((column, row)).unwrap();
                let cell = *cell as usize;

                let x_start = column * (cell_size + 1) + 1;
                let y_start = row * (cell_size + 1) + 1;

                let x_end = x_start + cell_size;
                let y_end = y_start + cell_size;

                [
                    // top-left part of the square
                    x_start, y_end, cell, x_start, y_start, cell, x_end, y_start, cell,
                    // bottom-right part of the square
                    x_end, y_start, cell, x_end, y_end, cell, x_start, y_end, cell,
                ]
            })
            .map(|value| value as u32)
            .collect_vec();
        console::time_end_with_label("Compute vertices");

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
            12,
            0,
        );
        context.enable_vertex_attrib_array(position_attribute_location as u32);

        context.vertex_attrib_i_pointer_with_i32(
            cell_attribute_location as u32,
            1,
            WebGl2RenderingContext::UNSIGNED_INT,
            12,
            8,
        );
        context.enable_vertex_attrib_array(cell_attribute_location as u32);

        context.bind_vertex_array(Some(&vao));

        let vertices_count = (vertices.len() / 3) as i32;

        context.clear_color(0.5, 0.5, 0.5, 1.0);
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vertices_count);

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
