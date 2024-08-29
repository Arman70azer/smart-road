use crate::matrix::{
    sub_mod_texture::{Texture, Textures},
    COLUMN, ROW,
};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};
use std::fmt;
pub mod sub_mod_path;
use sub_mod_path::{east_destination, north_destinations, south_destinations, west_destination};

#[derive(PartialEq,Clone,Copy)]
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
            Destinations::North => north_spawn(&destination, size),
            Destinations::South => south_spawn(&destination, size),
            Destinations::West => west_spawn(&destination, size),
            Destinations::East => east_spawn(&destination, size),
        };

        let row = position.0;
        let column = position.1;

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
            path, /*remplacer avec juste path */
            position,
            level_speed: 1,
            speed,
            size: sizy,
            choc: 0,
            destination,
        }
    }

    //Va chercher l'étape suivante puis redirige sur to_next_step pour mettre à jour row et column
    pub fn update_position(&mut self) {
        let mut next_step: Option<(i32, i32)> = None;

        for (index, position) in self.path.iter().enumerate() {
            if *position == self.position {
                // Vérifie s'il y a une prochaine étape dans le chemin
                if index + 1 < self.path.len() {
                    next_step = Some(self.path[index + 1]);
                }
                break;
            }
        }
        if let Some(step) = next_step {
            to_the_next_step(self, step)
        }
        // } else {
        //      println!("Vous êtes à la fin du chemin ou la position actuelle n'est pas trouvée.");
        //      println!("Si ce message apparait c'est qu'il n'y a pas assez d'étapes avant la fin du trajets => Il faut que la dernière étape est une valeur hors champs!!!!")
        // }
    }

    

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        self.texture
            .apply_texture(canvas, self.column, self.row, self.size)
    }
}

fn north_spawn(destination: &Destinations, cell_size: u32) -> (i32, i32) {
    if *destination == Destinations::West {
        return (0, 8*cell_size as i32);
    }
    if *destination == Destinations::South {
        return (0, 9*cell_size as i32);
    }
    (0, 10*cell_size as i32)
}

fn south_spawn(destination: &Destinations, cell_size: u32) -> (i32, i32) {
    if *destination == Destinations::West {
        return (ROW*cell_size as i32, 11*cell_size as i32);
    }
    if *destination == Destinations::North {
        return (ROW*cell_size as i32, 12*cell_size as i32);
    }
    (ROW*cell_size as i32, 13*cell_size as i32)
}

fn west_spawn(destination: &Destinations, cell_size: u32) -> (i32, i32) {
    if *destination == Destinations::North {
        return (11*cell_size as i32, 0);
    }
    if *destination == Destinations::East {
        return (12*cell_size as i32, 0);
    }
    (13*cell_size as i32, 0)
}

fn east_spawn(destination: &Destinations, cell_size: u32) -> (i32, i32) {
    if *destination == Destinations::North {
        return (8*cell_size as i32, COLUMN*cell_size as i32);
    }
    if *destination == Destinations::West {
        return (9*cell_size as i32, COLUMN*cell_size as i32);
    }
    (10*cell_size as i32, COLUMN*cell_size as i32)
}


fn to_the_next_step(car: &mut Car, next_step: (i32, i32)){

    let calcul_speed = (car.speed as i32) * car.level_speed;

    if car.row == next_step.0 && car.column == next_step.1{
        car.position = next_step;
    }else if car.position.0 == next_step.0 && car.position.1 != next_step.1{
        if car.position.1 > next_step.1{
            car.column-=calcul_speed;
        }else{
            car.column+= calcul_speed;
        }
    }else if car.position.0 != next_step.0 && car.position.1 == next_step.1{
        if car.position.0 > next_step.0{
            car.row-=calcul_speed;
        }else{
            car.row+=calcul_speed;
        }
    }
}