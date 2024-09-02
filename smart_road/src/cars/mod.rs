use crate::matrix::{
    sub_mod_texture::{Texture, Textures},
    COLUMN, ROW,
};
mod sub_mod_path;
use sub_mod_path::{east_destination, west_destination, north_destinations, south_destinations};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};
use std::fmt;
#[derive(PartialEq, Clone, Copy)]
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
    pub position: (i32, i32),
    pub level_speed: i32,
    pub speed: u32,
    pub size: u32,
    pub choc: i16,
    pub path: Vec<(i32, i32)>,
    pub destination: Destinations,
    pub index_path: u8,
    //Penser à mettre un temps,
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
        println!("Position: ({}, {}), Size: {}", position.0, position.1, size);
        let texture_type: Textures = match destination {
            Destinations::East => Textures::BlackCar,
            Destinations::West => Textures::OrangeCar,
            Destinations::North => Textures::BlueCar,
            Destinations::South => Textures::GreenCar,
        };
        let texture = Texture::new(texture_creator, &texture_type);
        //ICI il faut créer les fn destinations pour qu'il renvoie un Vec avec à l'intérieur les positions de
        //toutes les cases sur lesquelles la voiture devra ce rendre pour arriver à destination.
        let path = match destination {
            Destinations::South => south_destinations(row, column, size),
            Destinations::North => north_destinations(row, column, size),
            Destinations::East => east_destination(row, column, size),
            Destinations::West => west_destination(row, column, size),
        };
        let sizy = (size as f64 * 0.9) as u32;
        Car {
            row,
            column,
            texture,
            position,
            path,
            level_speed: 1,
            speed,
            size: sizy,
            choc: 0,
            destination,
            index_path: 0,
        }
    }
    
    pub fn update_position(&mut self) {
        if let Some(next_position) = self.path.get(self.index_path as usize + 1) {
            if next_position.0 == self.row && next_position.1 == self.column {
                self.position = (self.row, self.column);
                self.index_path += 1;
            } else {
                if next_position.0 != self.row {
                    if next_position.0 > self.row {
                        self.row += self.speed as i32;
                    } else {
                        self.row -= self.speed as i32;
                    }
                }
    
                if next_position.1 != self.column {
                    if next_position.1 > self.column {
                        self.column += self.speed as i32;
                    } else {
                        self.column -= self.speed as i32;
                    }
                }
            }
        }
    }
    
    pub fn change_direction(&mut self) {
        self.level_speed = 0;
        // Commencer à déplacer la voiture horizontalement (vers la droite)
        self.level_speed = 1;
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
            
            // Définir les rectangles de collision
            let rect_a = (
                car_a.column,
                car_a.row,
                car_a.size,
                car_a.size,
            );
            let rect_b = (
                car_b.column,
                car_b.row,
                car_b.size,
                car_b.size,
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
            car.level_speed = 0; // Ralentir la voiture
        }
    }
}