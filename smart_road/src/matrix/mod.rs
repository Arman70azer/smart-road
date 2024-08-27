// src/matrix/mod.rs

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

// Type alias for your Matrix type, assuming it's a 2D Vec of integers
pub type Matrix = Vec<Vec<i32>>;

pub fn draw_matrix(canvas: &mut Canvas<Window>, matrix: &Matrix, cell_size:u32) {
    // let rows = matrix.len();
    // let cols = if rows > 0 { matrix[0].len() } else { 0 };

    for (i, row) in matrix.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            let color = match value {
                0 => Color::RGB(255, 255, 255), // White for value 0
                1 => Color::RGB(0, 0, 255),     // Blue for value 1
                _ => Color::RGB(128, 128, 128), // Gray for any other value
            };

            let x = (j as u32) * cell_size;
            let y = (i as u32) * cell_size;

            canvas.set_draw_color(color);
            canvas.fill_rect(Rect::new(x as i32, y as i32, cell_size, cell_size)).unwrap();
        }
    }
}
