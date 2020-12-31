use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;


pub fn coordinate_conversion(value: i32, block_size: f64) -> f64 {
    (value as f64) * block_size
}

//External Usage: main.rs
pub fn u32_coordinate_conversion(value: i32, block_size: f64) -> u32 {
    coordinate_conversion(value, block_size) as u32
}

pub fn draw_block(color: Color, x: i32, y: i32, context: &Context, gfx: &mut G2d) {
    const BLOCK_SIZE: f64 = 25.0;

    let gui_x = coordinate_conversion(x, BLOCK_SIZE);
    let gui_y = coordinate_conversion(y, BLOCK_SIZE);

    rectangle(color, [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE], context.transform, gfx);
}

pub fn draw_rectangle(color: Color, x: i32, y: i32, width: i32, height: i32, context: &Context, gfx: &mut G2d) {
    const BLOCK_SIZE: f64 = 25.0;

    let x = coordinate_conversion(x, BLOCK_SIZE);
    let y = coordinate_conversion(y, BLOCK_SIZE);

    rectangle(color, [x, y, BLOCK_SIZE * (width as f64), BLOCK_SIZE * (height as f64)], context.transform, gfx);
}