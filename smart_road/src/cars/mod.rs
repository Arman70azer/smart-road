use crate::matrix::{sub_mod_texture::{Texture, Textures}, ROW, COLUMN};
use sdl2::video::{Window, WindowContext};
use sdl2::render::{TextureCreator, Canvas};
use std::fmt;

#[derive(PartialEq)]
pub enum Destinations {
    North,
    South,
    East,
    West,
}

pub struct Car<'a> {
    pub row: i32,
    pub column: i32,
    pub texture: Texture<'a>,
    pub path: Vec<(i32, i32)>,
    pub position: (i32, i32),
    pub level_speed: i32,
    pub speed: u32,
    pub size: u32,
    pub choc: i16,
}

impl<'a> fmt::Debug for Car<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Car")
            .field("row", &self.row)
            .field("column", &self.column)
            .field("path", &self.path)
            .field("position", &self.position)
            .field("level_speed", &self.level_speed)
            .field("speed", &self.speed)
            .field("size", &self.size)
            .finish()
    }
}

impl<'a> Car<'a> {
    pub fn new(spawn:Destinations, destination: Destinations, texture_creator: &'a TextureCreator<WindowContext>, speed : u32, size: u32)->Self{

        let position = match spawn {
            Destinations::North => north_spawn(&destination),
            Destinations::South => south_spawn(&destination),
            Destinations::West =>west_spawn(&destination),
            Destinations::East=> east_spawn(&destination),
        };

        let row = position.0; //* size as i32;
        let column = position.1; //* size as i32;
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
        let sizy = (size as f64 * 0.9) as u32;
        Car{row, column, texture, path: vec![(row, column)]/*juste pour le momment */, position, level_speed:1, speed, size: sizy, choc: 0 }
    }

    
    pub fn update_position(&mut self) {
        self.row += (self.speed as i32) * self.level_speed;
        self.position = (self.row, self.column);
        println!("{:?}",self);
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        self.texture.apply_texture(canvas, self.row, self.column, self.size)
    }
}

fn north_spawn(destination: &Destinations)->(i32, i32){
    if *destination == Destinations::West {
        return (0, 9);
    }
    if *destination == Destinations::South{
        return (0, 10);
    }
    (0, 11)
}

fn south_spawn(destination: &Destinations)->(i32, i32){
    if *destination == Destinations::West {
        return (ROW-1, 12);
    }
    if *destination== Destinations::South{
        return (ROW-1, 13);
    }
    (ROW-1, 14)
}

fn west_spawn(destination: &Destinations)->(i32, i32){
    if *destination == Destinations::West {
        return (9, 0);
    }
    if *destination== Destinations::South{
        return (10, 0);
    }
    (11, 0)
}

fn east_spawn(destination: &Destinations)->(i32, i32){
    if *destination == Destinations::West {
        return (9, COLUMN-1);
    }
    if *destination== Destinations::South{
        return (10, COLUMN-1);
    }
    (11,COLUMN-1)
}