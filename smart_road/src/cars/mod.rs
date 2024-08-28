use crate::matrix::{sub_mod_texture::Texture,ROW, COLUMN};
use sdl2::video::WindowContext;
use sdl2::render::TextureCreator;

#[derive(PartialEq)]
pub enum Destinations {
    North,
    South,
    East,
    West,
}

pub struct Car<'a> {
    pub row: u32,
    pub column: u32,
    pub texture: Texture<'a>,
    pub path: Vec<(u32, u32)>,
    pub position: (u32, u32),
    pub speed: u8,
}

impl<'a> Car<'a> {
    pub fn new(spawn:Destinations, destination: Destinations, texture_creator: &'a TextureCreator<WindowContext>)->Self{
        let positions = match spawn {
            Destinations::North => north_spawn(destination),
            Destinations::South => south_spawn(destination)
        }
        Car()
    }
}

fn north_spawn(destination: Destinations)->(u32, u32){
    if destination == Destinations::West {
        return (0, 9);
    }
    if destination== Destinations::South{
        return (0, 10);
    }
    (0, 11)
}

fn south_spawn(destination: Destinations)->(u32, u32){
    if destination == Destinations::West {
        return (COLUMN-1, 9);
    }
    if destination== Destinations::South{
        return (COLUMN-1, 10);
    }
    (0, 11)
}