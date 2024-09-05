use std::time::Instant;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use smart_road::cars::{sub_mod_cars::Cars, Destinations, Car};
use smart_road::utils::{random_cars, random_spawn, random_destinations};

const ACTION_INTERVAL: std::time::Duration = std::time::Duration::from_millis(800);
const SQUARE_SPEED: i32 = 1;

// Ajout de la contrainte de durée de vie
pub fn handle_keydown<'a, 'b: 'a>(
    keycode: sdl2::keyboard::Keycode,
    see_tab: &mut bool,
    last_action_time: &mut Instant,
    cars: &mut Cars<'a>,
    texture_creator: &'b TextureCreator<WindowContext>,
    cell_size: i32,
) {
    let now = Instant::now(); // Capturer l'instant présent pour utiliser la même valeur dans plusieurs vérifications

    match keycode {
        sdl2::keyboard::Keycode::Escape => std::process::exit(0),
        sdl2::keyboard::Keycode::S => {
            *see_tab = !*see_tab;
            *last_action_time = now;
        }
        sdl2::keyboard::Keycode::A => {
            cars.refresh();
            *last_action_time = now;
        }
        sdl2::keyboard::Keycode::Down if now.duration_since(*last_action_time) >= ACTION_INTERVAL && !*see_tab => {
            random_cars(Destinations::North, texture_creator, SQUARE_SPEED, cell_size, &mut cars.cars);
            *last_action_time = now; // Mise à jour de `last_action_time` pour respecter l'intervalle
        }
        sdl2::keyboard::Keycode::Up if now.duration_since(*last_action_time) >= ACTION_INTERVAL && !*see_tab => {
            random_cars(Destinations::South, texture_creator, SQUARE_SPEED, cell_size, &mut cars.cars);
            *last_action_time = now;
        }
        sdl2::keyboard::Keycode::Left if now.duration_since(*last_action_time) >= ACTION_INTERVAL && !*see_tab => {
            random_cars(Destinations::East, texture_creator, SQUARE_SPEED, cell_size, &mut cars.cars);
            *last_action_time = now;
        }
        sdl2::keyboard::Keycode::Right if now.duration_since(*last_action_time) >= ACTION_INTERVAL && !*see_tab => {
            random_cars(Destinations::West, texture_creator, SQUARE_SPEED, cell_size, &mut cars.cars);
            *last_action_time = now;
        }
        sdl2::keyboard::Keycode::R if now.duration_since(*last_action_time) >= ACTION_INTERVAL && !*see_tab => {
            random_cars(random_spawn(), texture_creator, SQUARE_SPEED, cell_size, &mut cars.cars);
            *last_action_time = now;
        }
        sdl2::keyboard::Keycode::P if now.duration_since(*last_action_time) >= ACTION_INTERVAL && !*see_tab => {
            let car_one = Car::new(Destinations::North, Destinations::South, texture_creator, SQUARE_SPEED as u32, cell_size as u32);
            let car_two = Car::new(Destinations::North, Destinations::East, texture_creator, SQUARE_SPEED as u32, cell_size as u32);
            let car_tree = Car::new(Destinations::North, Destinations::West, texture_creator, SQUARE_SPEED as u32, cell_size as u32);
            let car_four = Car::new(Destinations::South, random_destinations(Destinations::South), texture_creator, SQUARE_SPEED as u32, cell_size as u32);
            cars.cars.push(car_one);
            cars.cars.push(car_two);
            cars.cars.push(car_tree);
            cars.cars.push(car_four);
            *last_action_time = now;
        }
        sdl2::keyboard::Keycode::O if now.duration_since(*last_action_time) >= ACTION_INTERVAL && !*see_tab => {
            let car_one = Car::new(Destinations::South, Destinations::North, texture_creator, SQUARE_SPEED as u32, cell_size as u32);
            let car_two = Car::new(Destinations::South, Destinations::East, texture_creator, SQUARE_SPEED as u32, cell_size as u32);
            let car_tree = Car::new(Destinations::South, Destinations::West, texture_creator, SQUARE_SPEED as u32, cell_size as u32);
            let car_four = Car::new(Destinations::North, random_destinations(Destinations::North), texture_creator, SQUARE_SPEED as u32, cell_size as u32);
            cars.cars.push(car_one);
            cars.cars.push(car_two);
            cars.cars.push(car_tree);
            cars.cars.push(car_four);
            *last_action_time = now;
        }
        sdl2::keyboard::Keycode::I if now.duration_since(*last_action_time) >= ACTION_INTERVAL && !*see_tab => {
            let car_one = Car::new(Destinations::East, Destinations::South, texture_creator, SQUARE_SPEED as u32, cell_size as u32);
            let car_two = Car::new(Destinations::East, Destinations::West, texture_creator, SQUARE_SPEED as u32, cell_size as u32);
            let car_tree = Car::new(Destinations::East, Destinations::North, texture_creator, SQUARE_SPEED as u32, cell_size as u32);
            let car_four = Car::new(Destinations::West, random_destinations(Destinations::West), texture_creator, SQUARE_SPEED as u32, cell_size as u32);
            cars.cars.push(car_one);
            cars.cars.push(car_two);
            cars.cars.push(car_tree);
            cars.cars.push(car_four);
            *last_action_time = now;
        }
        sdl2::keyboard::Keycode::U if now.duration_since(*last_action_time) >= ACTION_INTERVAL && !*see_tab => {
            let car_one = Car::new(Destinations::West, Destinations::South, texture_creator, SQUARE_SPEED as u32, cell_size as u32);
            let car_two = Car::new(Destinations::West, Destinations::East, texture_creator, SQUARE_SPEED as u32, cell_size as u32);
            let car_tree = Car::new(Destinations::West, Destinations::North, texture_creator, SQUARE_SPEED as u32, cell_size as u32);
            let car_four = Car::new(Destinations::East, random_destinations(Destinations::East), texture_creator, SQUARE_SPEED as u32, cell_size as u32);
            cars.cars.push(car_one);
            cars.cars.push(car_two);
            cars.cars.push(car_tree);
            cars.cars.push(car_four);
            *last_action_time = now;
        }
        sdl2::keyboard::Keycode::M if now.duration_since(*last_action_time) >= ACTION_INTERVAL && !*see_tab => {
            let car_one = Car::new(Destinations::South, Destinations::North, texture_creator, SQUARE_SPEED as u32, cell_size as u32);
            let car_two = Car::new(Destinations::South, Destinations::East, texture_creator, SQUARE_SPEED as u32, cell_size as u32);
            let car_tree = Car::new(Destinations::South, Destinations::West, texture_creator, SQUARE_SPEED as u32, cell_size as u32);
            let car_four = Car::new(Destinations::West, random_destinations(Destinations::West), texture_creator, SQUARE_SPEED as u32, cell_size as u32);
            cars.cars.push(car_one);
            cars.cars.push(car_two);
            cars.cars.push(car_tree);
            cars.cars.push(car_four);
            *last_action_time = now;
        }
        sdl2::keyboard::Keycode::L if now.duration_since(*last_action_time) >= ACTION_INTERVAL && !*see_tab => {
            let car_one = Car::new(Destinations::South, Destinations::North, texture_creator, SQUARE_SPEED as u32, cell_size as u32);
            let car_two = Car::new(Destinations::South, Destinations::East, texture_creator, SQUARE_SPEED as u32, cell_size as u32);
            let car_tree = Car::new(Destinations::South, Destinations::West, texture_creator, SQUARE_SPEED as u32, cell_size as u32);
            let car_four = Car::new(Destinations::East, random_destinations(Destinations::East), texture_creator, SQUARE_SPEED as u32, cell_size as u32);
            cars.cars.push(car_one);
            cars.cars.push(car_two);
            cars.cars.push(car_tree);
            cars.cars.push(car_four);
            *last_action_time = now;
        }
        sdl2::keyboard::Keycode::K if now.duration_since(*last_action_time) >= ACTION_INTERVAL && !*see_tab => {
            let car_one = Car::new(Destinations::North, Destinations::South, texture_creator, SQUARE_SPEED as u32, cell_size as u32);
            let car_two = Car::new(Destinations::North, Destinations::East, texture_creator, SQUARE_SPEED as u32, cell_size as u32);
            let car_tree = Car::new(Destinations::North, Destinations::West, texture_creator, SQUARE_SPEED as u32, cell_size as u32);
            let car_four = Car::new(Destinations::East, random_destinations(Destinations::East), texture_creator, SQUARE_SPEED as u32, cell_size as u32);
            cars.cars.push(car_one);
            cars.cars.push(car_two);
            cars.cars.push(car_tree);
            cars.cars.push(car_four);
            *last_action_time = now;
        }
        sdl2::keyboard::Keycode::J if now.duration_since(*last_action_time) >= ACTION_INTERVAL && !*see_tab => {
            let car_one = Car::new(Destinations::North, Destinations::South, texture_creator, SQUARE_SPEED as u32, cell_size as u32);
            let car_two = Car::new(Destinations::North, Destinations::East, texture_creator, SQUARE_SPEED as u32, cell_size as u32);
            let car_tree = Car::new(Destinations::North, Destinations::West, texture_creator, SQUARE_SPEED as u32, cell_size as u32);
            let car_four = Car::new(Destinations::West, random_destinations(Destinations::West), texture_creator, SQUARE_SPEED as u32, cell_size as u32);
            cars.cars.push(car_one);
            cars.cars.push(car_two);
            cars.cars.push(car_tree);
            cars.cars.push(car_four);
            *last_action_time = now;
        }
        
        

        _ => {}
    }
}
