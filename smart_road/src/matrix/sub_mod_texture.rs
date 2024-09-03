use sdl2::render::{Texture as SdlTexture, TextureCreator, Canvas};
use sdl2::video::WindowContext;
use sdl2::rect::{Rect, Point};
use sdl2::image::LoadTexture;
use std::fmt;

pub enum Textures {
    Herbe,
    RoadRow,
    RoadCol,
    RoadCent,
    BlackCar,
    OrangeCar,
    GreenCar,
    BlueCar,
}

pub struct Texture<'a> {
    pub texture: SdlTexture<'a>,
}

// Implémentation Debug pour Texture si possible
impl<'a> fmt::Debug for Texture<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Texture")
            .field("texture", &"Texture data") // Placeholder: replace with actual data if needed
            .finish()
    }
}

impl<'a> Texture<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>, texture_type: &Textures) -> Self {
        let filename = match texture_type {
            Textures::Herbe => "./src/images/herbes.png",
            Textures::RoadRow => "./src/images/road_east_west.png",
            Textures::RoadCol => "./src/images/road_north_south.png",
            Textures::RoadCent => "./src/images/center.png",
            Textures::BlackCar => "./src/images/car_black.png",
            Textures::OrangeCar => "./src/images/car_red.png",
            Textures::BlueCar => "./src/images/car_blue.png",
            Textures::GreenCar => "./src/images/car_green.png",

        };

        let texture = texture_creator.load_texture(filename).unwrap();
        Texture { texture }
    }

    pub fn apply_texture(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, x: i32, y: i32, cell_size: u32) {
        let dest_rect = Rect::new(x as i32, y as i32, cell_size, cell_size);
        canvas.copy(&self.texture, None, Some(dest_rect)).unwrap();
    }

    pub fn apply_texture_with_rotation(&self, canvas: &mut Canvas<sdl2::video::Window>, x: i32, y: i32, cell_size: u32, rotation: f64) {
        let dest_rect = Rect::new(x, y, cell_size, cell_size);
        let center = Some(Point::new(cell_size as i32 / 2, cell_size as i32 / 2));
        canvas.copy_ex(&self.texture, None, Some(dest_rect), rotation, center, false, false).unwrap();
    }
}

