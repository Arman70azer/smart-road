use crate::matrix::{sub_mod_texture::{Texture, Textures}, ROW, COLUMN};
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

        let position = match spawn {
            Destinations::North => north_spawn(&destination),
            Destinations::South => south_spawn(&destination),
            Destinations::West =>west_spawn(&destination),
            Destinations::East=> east_spawn(&destination),
        };

        let row = position.0;
        let column = position.1;
        let texture_type = match destination {
            Destinations::East=> Textures::BlackCar,
            Destinations::West => Textures::OrangeCar,
            Destinations::North => Textures::BlueCar,
            Destinations::South => Textures::GreenCar,
        };

        let texture = Texture::new(texture_creator, &texture_type);

        // let path = match destination {
        //     Destinations::South => south_destinations(positions),
        //     Destinations::North => north_destinations(positions),
        //     Destinations::East => east_destinations(positions),
        //     Destinations::West => west_destinations(positions),
        // };
        Car{row, column, texture, path: vec![(row, column)]/*juste pour le momment */, position, speed:1 }
    }
}

fn north_spawn(destination: &Destinations)->(u32, u32){
    if *destination == Destinations::West {
        return (0, 9);
    }
    if *destination == Destinations::South{
        return (0, 10);
    }
    (0, 11)
}

fn south_spawn(destination: &Destinations)->(u32, u32){
    if *destination == Destinations::West {
        return (ROW-1, 12);
    }
    if *destination== Destinations::South{
        return (ROW-1, 13);
    }
    (ROW-1, 14)
}

fn west_spawn(destination: &Destinations)->(u32, u32){
    if *destination == Destinations::West {
        return (9, 0);
    }
    if *destination== Destinations::South{
        return (10, 0);
    }
    (11, 0)
}

fn east_spawn(destination: &Destinations)->(u32, u32){
    if *destination == Destinations::West {
        return (9, COLUMN-1);
    }
    if *destination== Destinations::South{
        return (10, COLUMN-1);
    }
    (11,COLUMN-1)
}