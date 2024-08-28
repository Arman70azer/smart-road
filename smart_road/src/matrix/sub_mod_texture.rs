use sdl2::render::{Texture as SdlTexture, TextureCreator};
use sdl2::video::WindowContext;
use sdl2::rect::Rect;
use sdl2::image::LoadTexture;

#[derive(Copy, Clone)]
pub enum Textures {
    Herbe,
    RoadRow,
    RoadCol,
}

pub struct Texture<'a> {
    pub texture: SdlTexture<'a>,
}

impl<'a> Texture<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>, texture_type: &Textures) -> Self {
        let filename = match texture_type {
            Textures::Herbe => "./src/images/herbes.png",
            Textures::RoadRow => "./src/images/road_east_west.png",
            Textures::RoadCol => "./src/images/road_north_south.png",
        };

        let texture = texture_creator.load_texture(filename).unwrap();
        Texture { texture }
    }

    pub fn apply_texture(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, x: u32, y: u32, cell_size: u32) {
        let dest_rect = Rect::new(x as i32, y as i32, cell_size, cell_size);
        canvas.copy(&self.texture, None, Some(dest_rect)).unwrap();
   