use crate::matrix::{
    sub_mod_texture::{Texture, Textures},
    COLUMN, ROW,
};
mod sub_mod_path;
pub mod sub_mod_cars;
use rand::Rng;
use sub_mod_path::{east_destination, west_destination, north_destinations, south_destinations};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};
use std::{fmt, time::{Instant,Duration}};
#[derive(PartialEq, Clone, Copy)]
pub enum Destinations {
    North,
    South,
    East,
    West,
}

impl Destinations {
    pub fn to_degrees(&self) -> f32 {
        match self {
            Destinations::North => 0.0,
            Destinations::East => 90.0,
            Destinations::South => 180.0,
            Destinations::West => 270.0,
        }
    }

    pub fn to_radians(&self) -> f32 {
        self.to_degrees().to_radians()
    }
}


pub struct Car<'a> {
    pub row: i32,
    pub column: i32,
    pub texture: Texture<'a>,
    pub position: (i32, i32),
    pub level_speed: i32,
    pub speed: u32,
    pub size: u32,
    pub choc: i16,
    pub path: Vec<(i32, i32)>,
    pub destination: Destinations,
    pub index_path: usize,
    pub last_update: Instant, 
    pub timer: Duration,
    pub collision_extension_midlle: i32,
    pub collision_extension_low: i32,
    pub name: i32,
}

impl<'a> fmt::Debug for Car<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Car")
            .field("row", &self.row)
            .field("column", &self.column)
            //.field("path", &self.path)
            .field("position", &self.position)
            .field("level_speed", &self.level_speed)
            .field("speed", &self.speed)
            .field("size", &self.size)
            .finish()
    }
}

impl<'a> Car<'a> {
    pub fn new(
        spawn: Destinations,
        destination: Destinations,
        texture_creator: &'a TextureCreator<WindowContext>,
        speed: u32,
        size: u32,
    ) -> Self {
        let position = match spawn {
            Destinations::North => north_spawn(&destination),
            Destinations::South => south_spawn(&destination),
            Destinations::West => west_spawn(&destination),
            Destinations::East => east_spawn(&destination),
        };
        let row = position.0 * size as i32;
        let column = position.1 * size as i32;
    
        let texture_type: Textures = match destination {
            Destinations::East => Textures::BlackCar,
            Destinations::West => Textures::OrangeCar,
            Destinations::North => Textures::BlueCar,
            Destinations::South => Textures::GreenCar,
        };
        let texture = Texture::new(texture_creator, &texture_type);
        
        let path = match destination {
            Destinations::South => south_destinations(row, column, size),
            Destinations::North => north_destinations(row, column, size),
            Destinations::East => east_destination(row, column, size),
            Destinations::West => west_destination(row, column, size),
        };

        let mut rng = rand::thread_rng();
        Car {
            row,
            column,
            texture,
            position,
            path,
            level_speed: 1,
            speed,
            size,
            choc: 0,
            destination,
            index_path: 0,
            last_update: Instant::now(),
            timer: Duration::new(0, 0),
            collision_extension_midlle: 50,
            collision_extension_low: 10,
            name: rng.gen_range(1_000_000..10_000_000),
        }
    }
    
    pub fn update_position(&mut self) {

        if let Some(next_position) = self.path.get(self.index_path as usize + 1) {
            if next_position.0 != self.row {
                if next_position.0 > self.row {
                    self.row += (self.speed as i32) * self.level_speed;
                    self.destination = Destinations::South;

                    if self.row >= next_position.0{
                        self.position = *next_position;
                        self.row = self.position.0;
                        self.column = self.position.1;
                        self.index_path += 1;
                        if let Some(destination) = where_we_go(self) {
                            self.destination = destination;
                        }
                    }

                } else {
                    self.row -= (self.speed as i32) * self.level_speed;
                    self.destination = Destinations::North;

                    if self.row <= next_position.0{
                        self.position = *next_position;
                        self.row = self.position.0;
                        self.column = self.position.1;
                        self.index_path += 1;
                        if let Some(destination) = where_we_go(self) {
                            self.destination = destination;
                        }
                    }
                }
            }else{
                if next_position.1 > self.column {
                    self.column += (self.speed as i32) * self.level_speed;
                    self.destination = Destinations::East;

                    if self.column >= next_position.1{
                        self.position = *next_position;
                        self.row = self.position.0;
                        self.column = self.position.1;
                        self.index_path += 1;
                        if let Some(destination) = where_we_go(self) {
                            self.destination = destination;
                        }

                    }
                } else {
                    self.column -= (self.speed as i32) * self.level_speed;
                    self.destination = Destinations::West;

                    if self.column <= next_position.1{
                        self.position = *next_position;
                        self.row = self.position.0;
                        self.column = self.position.1;
                        self.index_path += 1;
                        if let Some(destination) = where_we_go(&self) {
                            self.destination = destination;
                        }
                    }
                }
            }
        }else{
            println!("Erreur position!!!!!!!!");
            println!("ici: {}", self.level_speed);
            println!("index: {}", self.index_path);
            println!("positions: {:?}", self.position);
            println!("path: {:?}", self.path);
        }
    }
    
    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        let rotation = self.destination.to_degrees() as f64;
        self.texture
            .apply_texture_with_rotation(canvas, self.column, self.row, self.size, rotation)
    }
}

fn north_spawn(destination: &Destinations) -> (i32, i32) {
    if *destination == Destinations::West {
        return (0, 8);
    }
    if *destination == Destinations::South {
        return (0, 9);
    }
    (0, 10)
}

fn south_spawn(destination: &Destinations) -> (i32, i32) {
    if *destination == Destinations::West {
        return (ROW, 11);
    }
    if *destination == Destinations::North {
        return (ROW, 12);
    }
    (ROW, 13)
}

fn west_spawn(destination: &Destinations) -> (i32, i32) {
    if *destination == Destinations::North {
        return (11, 0);
    }
    if *destination == Destinations::East {
        return (12, 0);
    }
    (13, 0)
}

fn east_spawn(destination: &Destinations) -> (i32, i32) {
    if *destination == Destinations::North {
        return (8, COLUMN);
    }
    if *destination == Destinations::West {
        return (9, COLUMN);
    }
    (10, COLUMN)
}

fn where_we_go(car: &Car) -> Option<Destinations> {
    if let Some(next_position) = car.path.get(car.index_path as usize + 1) {
        if next_position.0 != car.row {
            if car.row > next_position.0 {
                return Some(Destinations::North);
            } else {
                return Some(Destinations::South);
            }
        } else {
            if car.column > next_position.1 {
                return Some(Destinations::West);
            } else {
                return Some(Destinations::East);
            }
        }
    }
    
    None // Retourne None si la prochaine position n'existe pas
}

