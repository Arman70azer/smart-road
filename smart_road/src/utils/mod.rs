use crate::cars::{Car, Destinations};
use sdl2::render::TextureCreator;
use rand::{prelude::SliceRandom, Rng};


// pub fn applicate_texture(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>, filename: String, heigth: u32, width: u32) {
//     // Load a texture from an image file
//     let texture = texture_creator.load_texture(filename).unwrap();

//     // Define the destination rectangle (where on the canvas to draw the texture)
//     let dest_rect = Rect::new(0, 0, width, heigth); // Example position and size

//     // Clear the canvas to white before drawing
//     canvas.set_draw_color(Color::RGB(255, 255, 255));
//     canvas.clear();

//     // Copy the texture to the canvas
//     canvas.copy(&texture, None, Some(dest_rect)).unwrap();
// }

pub fn random_cars<'a>(
    spawn: Destinations,
    texture_creator: &'a TextureCreator<sdl2::video::WindowContext>,
    square_speed: i32,
    cell_size: i32,
    cars: &mut Vec<Car<'a>>,
) {
    let new_car = Car::new(spawn, random_destinations(spawn), texture_creator,
        square_speed as u32,
        cell_size as u32,
    );
    cars.push(new_car);
}

pub fn random_destinations(spawn: Destinations) -> Destinations {
    let mut rng = rand::thread_rng(); // Création d'un générateur de nombres aléatoires

    match spawn {
        Destinations::North => {
            // Choisir aléatoirement entre South, East, West
            *[Destinations::South, Destinations::East, Destinations::West]
                .choose(&mut rng)
                .expect("Failed to choose a random destination")
        }
        Destinations::South => {
            // Choisir aléatoirement entre North, East, West
            *[Destinations::North, Destinations::East, Destinations::West]
                .choose(&mut rng)
                .expect("Failed to choose a random destination")
        }
        Destinations::East => {
            // Choisir aléatoirement entre North, South, West
            *[Destinations::North, Destinations::South, Destinations::West]
                .choose(&mut rng)
                .expect("Failed to choose a random destination")
        }
        Destinations::West => {
            // Choisir aléatoirement entre North, South, East
            *[Destinations::North, Destinations::South, Destinations::East]
                .choose(&mut rng)
                .expect("Failed to choose a random destination")
        }
    }
}

pub fn random_spawn() -> Destinations {
    let mut rng = rand::thread_rng(); 
    let num_random = rng.gen_range(0..4); 

    match num_random {
       0 => Destinations::East,
       1 => Destinations::West,
       2 => Destinations::North,
       3 => Destinations::South,
       _ => unreachable!(), 
    }
}
