use crate::matrix::{
    sub_mod_texture::{Texture, Textures},
    COLUMN, ROW,
};
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
    pub path: Vec<(i32, i32)>,
    pub position: (i32, i32),
    pub level_speed: i32,
    pub speed: u32,
    pub size: u32,
    pub choc: i16,
    pub destination: Destinations,
    //Penser à mettre un temps,
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
        // let path = match destination {
        //     Destinations::South => south_destinations(positions),
        //     Destinations::North => north_destinations(positions),
        //     Destinations::East => east_destinations(positions),
        //     Destinations::West => west_destinations(positions),
        // };
        let sizy = (size as f64 * 0.9) as u32;
        Car {
            row,
            column,
            texture,
            path: vec![(row, column)], /*remplacer avec juste path */
            position,
            level_speed: 1,
            speed,
            size: sizy,
            choc: 0,
            destination,
        }
    }
    //Ici Il faut de préfèrence finir d'apporter le path à la voiture avant de commencer
    //la voiture devra ce déplacer à l'étape suivante en utilsant comme réfèrence la car.position et en cherchant l'étape suivante dans car.path
    // Déplacer la voiture en fonction de sa vitesse et direction actuelle
    pub fn update_position(&mut self) {
        match self.destination {
            Destinations::East => {
                // BON
                if self.position.0 < 398 {
                    // 395
                    self.row += (self.speed as i32) * self.level_speed;
                    self.position = (self.row, self.column);
                    println!("lalala");
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
                    // 395
                    self.row += (self.speed as i32) * self.level_speed;
                    self.position = (self.row, self.column);
                    println!("lalala");
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

        println!("{:?}", self);
    }

    // Définir change_direction comme une méthode d'instance de Car
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
        return (ROW - 1, 11);
    }
    if *destination == Destinations::North {
        return (ROW - 1, 12);
    }
    (ROW - 1, 13)
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
        return (8, COLUMN - 1);
    }
    if *destination == Destinations::West {
        return (9, COLUMN - 1);
    }
    (10, COLUMN - 1)
}
