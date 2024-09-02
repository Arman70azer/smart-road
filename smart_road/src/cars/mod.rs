use crate::matrix::{
    sub_mod_texture::{Texture, Textures},
    COLUMN, ROW,
};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};
use std::fmt;
// use std::f32::consts::PI;

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
    pub path: Vec<(i32, i32)>,
    pub position: (i32, i32),
    pub level_speed: i32,
    pub speed: u32,
    pub size: u32,
    pub choc: i16,
    pub destination: Destinations,
    pub collision_extension: i32,
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
        // println!("Position: ({}, {}), Size: {}", position.0, position.1, size);
        let texture_type: Textures = match destination {
            Destinations::East => Textures::BlackCar,
            Destinations::West => Textures::OrangeCar,
            Destinations::North => Textures::BlueCar,
            Destinations::South => Textures::GreenCar,
        };
        let texture = Texture::new(texture_creator, &texture_type);
        let sizy = (size as f64 * 0.9) as u32;
        Car {
            row,
            column,
            texture,
            path: vec![(row, column)],
            position,
            level_speed: 4,
            speed,
            size: sizy,
            choc: 0,
            destination,
            collision_extension: 50,
        }
    }

    pub fn update_position(&mut self) {
        match self.destination {
            Destinations::East => {
                if self.position.0 < 398 {
                    self.row += (self.speed as i32) * self.level_speed;
                    self.position = (self.row, self.column);
                } else if self.position.0 == 398 && self.column >= 360 {
                    self.change_direction();
                    self.column += (self.speed as i32) * self.level_speed;
                } else if self.position.0 > 470 {
                    self.row -= (self.speed as i32) / self.level_speed;
                    self.position = (self.row, self.column);
                } else if self.position.0 <= 470 && self.column >= 360 {
                    self.change_direction();
                    self.column += (self.speed as i32) * self.level_speed;
                } else {
                    self.column += (self.speed as i32) * self.level_speed;
                    self.position = (self.row, self.column);
                }
            }
            Destinations::North => {
                if self.position.1 < 398 {
                    self.column += (self.speed as i32) * self.level_speed;
                    self.position = (self.row, self.column);
                } else if self.position.1 == 398 && self.row >= 360 {
                    self.change_direction();
                    self.row -= (self.speed as i32) * self.level_speed;
                } else if self.position.1 > 470 {
                    self.column -= (self.speed as i32) / self.level_speed;
                    self.position = (self.row, self.column);
                } else if self.position.1 <= 470 && self.row >= 360 {
                    self.change_direction();
                    self.row -= (self.speed as i32) * self.level_speed;
                } else {
                    self.row -= (self.speed as i32) * self.level_speed;
                    self.position = (self.row, self.column);
                }
            }
            Destinations::South => {
                if self.position.1 < 288 {
                    self.column += (self.speed as i32) * self.level_speed;
                    self.position = (self.row, self.column);
                } else if self.position.1 == 288 && self.row >= 360 {
                    self.change_direction();
                    self.row += (self.speed as i32) * self.level_speed;
                } else if self.position.1 > 362 {
                    self.column -= (self.speed as i32) / self.level_speed;
                    self.position = (self.row, self.column);
                } else if self.position.1 <= 362 && self.row >= 360 {
                    self.change_direction();
                    self.row += (self.speed as i32) * self.level_speed;
                } else {
                    self.row += (self.speed as i32) * self.level_speed;
                    self.position = (self.row, self.column);
                }
            }
            Destinations::West => {
                if self.position.0 < 288 {
                    self.row += (self.speed as i32) * self.level_speed;
                    self.position = (self.row, self.column);
                    // println!("lalala");
                } else if self.position.0 == 288 && self.column >= 360 {
                    self.change_direction();
                    self.column += (self.speed as i32) * self.level_speed;
                } else if self.position.0 > 362 {
                    self.row -= (self.speed as i32) / self.level_speed;
                    self.position = (self.row, self.column);
                } else if self.position.0 <= 362 && self.column >= 360 {
                    self.change_direction();
                    self.column -= (self.speed as i32) * self.level_speed;
                } else {
                    self.column -= (self.speed as i32) * self.level_speed;
                    self.position = (self.row, self.column);
                }
            }
        }

        // println!("{:?}", self);
    }

    pub fn change_direction(&mut self) {
        self.level_speed = 0;
        self.level_speed = 4;
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        self.texture
            .apply_texture(canvas, self.column, self.row, self.size)
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

pub fn detect_collisions(cars: &mut [Car]) -> Vec<(usize, usize)> {
    let mut collisions = Vec::new();
    
    for i in 0..cars.len() {
        for j in i + 1..cars.len() {
            let car_a = &cars[i];
            let car_b = &cars[j];
            
            // Définir les rectangles de collision avec l'extension vers l'avant
            let (rect_a, rect_b) = (
                expand_collision_rect(car_a),
                expand_collision_rect(car_b),
            );
            
            // Vérifier le chevauchement des rectangles
            if rect_a.0 < rect_b.0 + (rect_b.2 as i32)
                && rect_a.0 + (rect_a.2 as i32) > rect_b.0
                && rect_a.1 < rect_b.1 + (rect_b.3 as i32)
                && rect_a.1 + (rect_a.3 as i32) > rect_b.1
            {
                collisions.push((i, j));
            }
        }
    }
    
    collisions
}

fn expand_collision_rect(car: &Car) -> (i32, i32, i32, i32) {
    let radians = car.destination.to_radians();
    let dx = (radians.cos() * car.collision_extension as f32) as i32;
    let dy = (radians.sin() * car.collision_extension as f32) as i32;

    // Calculer l'extension de collision en fonction de la direction
    let extension_x = dx;
    let extension_y = dy;
    
    (
        car.column - (car.size as i32 / 2) + extension_x,
        car.row - (car.size as i32 / 2) + extension_y,
        car.size as i32 + car.collision_extension as i32,
        car.size as i32 + car.collision_extension as i32,
    )
}

pub fn update_cars(cars: &mut [Car]) {
    for car in cars.iter_mut() {
        if car.level_speed > 0 {
            car.update_position();
        }
    }
}

pub fn handle_collisions(cars: &mut [Car], collisions: Vec<(usize, usize)>) {
    let mut slow_down_cars = std::collections::HashSet::new();

    for (i, j) in collisions {
        slow_down_cars.insert(i);
        slow_down_cars.insert(j);
    }

    for car_index in slow_down_cars {
        if let Some(car) = cars.get_mut(car_index) {
            car.level_speed = 2; // Ralentir la voiture
        }
    }
}
