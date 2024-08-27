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
        Cell { texture, row: 0, column: 0, size:0 }
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

fn draw_matrix_in_canva(canvas: &mut Canvas<Window>, matrix: &mut Matrix, cell_size:u32) {
    for (i, row) in matrix.iter_mut().enumerate() {  // `iter_mut` pour itérer avec des références mutables
        for (j, cell) in row.iter_mut().enumerate() {
            let x = (j as u32) * cell_size;
            let y: u32 = (i as u32) * cell_size;
            cell.texture.apply_texture(canvas, x, y, cell_size);
            cell.change_coordination(x, y); 
            cell.change_size(cell_size);
        }
    }
}



pub fn matrix_and_canva<'a>(canvas: &mut Canvas<Window>, heigth: u32, width: u32){
    // Example matrix to draw
    // Example matrix to draw
    let texture_creator = canvas.texture_creator();
    let road_texture: Texture<'_> = Texture::new(&texture_creator, &Textures::Road);
    let herbe_texture: Texture<'_> = Texture::new(&texture_creator, &Textures::Herbe);

    let mut matrix: Matrix = vec![
        vec![Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture)],
        vec![Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture)],
        vec![Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture)],
        vec![Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture)],
        vec![Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture)],
        vec![Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture)],
        vec![Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture)],
        vec![Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture)],
        vec![Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture)],
        vec![Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture)],
        vec![Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture)],
        vec![Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture)],
        vec![Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture),Cell::new(&herbe_texture), Cell::new(&road_texture)],
    ];
    

    let num_of_rows = matrix.len();
    let num_of_cols = if num_of_rows > 0 { matrix[0].len() } else { 0 };
    println!("{}", num_of_rows);

    // Calculate cell size based on canvas dimensions
    let cell_size_width = width / num_of_cols as u32;
    let cell_size_height = heigth / num_of_rows as u32;

    // Use the smaller of the two dimensions to ensure cells fit in the canvas
    let cell_size: u32 = cell_size_width.min(cell_size_height);

    draw_matrix_in_canva(canvas, &mut matrix, cell_size);
}