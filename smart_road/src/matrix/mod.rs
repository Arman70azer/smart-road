// src/matrix/mod.rs
pub mod sub_mod_texture;
use sub_mod_texture::{Texture, Textures};
use sdl2::render::Canvas;
use sdl2::video::Window;

#[derive(Copy, Clone)]
pub struct Cell<'a> {
    pub texture: &'a Texture<'a>,
    pub row: u32,
    pub column: u32,
    pub size: u32,
}

impl<'a> Cell<'a> {
    pub fn new(texture: &'a Texture<'a>) -> Self {
        Cell { texture, row: 0, column: 0, size: 0 }
    }

    pub fn change_coordination(&mut self, row: u32, column: u32) {
        self.row = row;
        self.column = column;
    }

    pub fn change_size(&mut self, size: u32){
        self.size=size;
    }
}

pub type Matrix<'a> = Vec<Vec<Cell<'a>>>;

pub fn draw_matrix_in_canva(canvas: &mut Canvas<Window>, matrix: &Matrix, cell_size: i32) {
    for (i, row) in matrix.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            let x: i32 = (j as i32) * cell_size;
            let y: i32 = (i as i32) * cell_size;
            cell.texture.apply_texture(canvas, x, y, cell_size as u32);
        }
    }
}


pub const ROW: i32 = 22;
pub const COLUMN: i32 = 22; 

pub fn matrix_and_canva<'a>(
    canvas: &mut Canvas<Window>,
    height: i32,
    width: i32
) {

    let cell_size_width = width / COLUMN;
    let cell_size_height = height / ROW ;


    let cell_size = cell_size_width.min(cell_size_height);

    let texture_creator = canvas.texture_creator();
    let road_row_texture = Texture::new(&texture_creator, &Textures::RoadRow);
    let road_col_texture = Texture::new(&texture_creator, &Textures::RoadCol);
    let road_cent_texture = Texture::new(&texture_creator, &Textures::RoadCent);
    let herbe_texture = Texture::new(&texture_creator, &Textures::Herbe);

    // Initialize the matrix with `Cell`s
    let mut matrix: Matrix = vec![vec![]; ROW as usize];
    for row in 0..ROW {
        for col in 0..COLUMN {
            let texture = 
            if row >= 8 && row <= 13 && col < 8 || row >= 8 && row <= 13 && col > 13 { 
                &road_row_texture
            } else if col >= 8 && col <= 13 && row < 8 || col >= 8 && col <= 13 && row > 13  {
                &road_col_texture
            } else if row >= 8 && row <= 13 && col >= 8 && col <= 13 {
                &road_cent_texture
            } else { 
                &herbe_texture
            };
            matrix[row as usize].push(Cell::new(texture));
        }
    }

    // Draw the matrix on the canvas
    draw_matrix_in_canva(canvas, &mut matrix, cell_size);
}
