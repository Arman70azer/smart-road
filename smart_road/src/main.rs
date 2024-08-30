extern crate sdl2;
use std::time::Duration;
use smart_road::matrix::{matrix_and_canva, ROW, COLUMN}; // Import the draw_matrix function
use smart_road::cars::{Car, Destinations};
use sdl2::image::InitFlag;
use sdl2::pixels::Color;
const WIDTH: i32 = 800; // Example width
const HEIGHT: i32 = 800; // Example height
const SQUARE_SPEED: i32= 1;
fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    sdl2::image::init(InitFlag::PNG | InitFlag::JPG | InitFlag::WEBP).unwrap();
    let window = video_subsystem
        .window("SDL2 Window", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    let texture_creator = canvas.texture_creator();
    matrix_and_canva(&mut canvas ,HEIGHT, WIDTH );
    
    canvas.present();
    let cell_size_width = WIDTH / COLUMN;
    let cell_size_height = HEIGHT / ROW;
    let cell_size = cell_size_width.min(cell_size_height);
    // Main loop placeholder
    let mut event_pump = sdl_context.event_pump().unwrap();
     // Créer un vecteur pour stocker les voitures
     let mut cars: Vec<Car> = Vec::new();
     'running: loop {
         // Gestion des événements
         for event in event_pump.poll_iter() {
             match event {
                 sdl2::event::Event::Quit { .. } => break 'running,
                 sdl2::event::Event::KeyDown { keycode, .. } => {
                        match keycode {
                            Some(sdl2::keyboard::Keycode::Escape) => break 'running,
                            Some(sdl2::keyboard::Keycode::Down) => {
                                smart_road::utils::random_cars(Destinations::North, &texture_creator, SQUARE_SPEED, cell_size, &mut cars);
                            }
                            Some(sdl2::keyboard::Keycode::Up) => {
                                smart_road::utils::random_cars(Destinations::South, &texture_creator, SQUARE_SPEED, cell_size, &mut cars);
                            }
                            Some(sdl2::keyboard::Keycode::Left) => {
                                smart_road::utils::random_cars(Destinations::East, &texture_creator, SQUARE_SPEED, cell_size, &mut cars);
                            }
                            Some(sdl2::keyboard::Keycode::Right) => {
                                smart_road::utils::random_cars(Destinations::West, &texture_creator, SQUARE_SPEED, cell_size, &mut cars);
                            }
                            _ => {}
                        } 
                 }
                 _ => {}
             }
         }
 
         // Mettre à jour la position de chaque voitures
         for car in &mut cars{
             car.update_position();
         }
         cars.retain(|car| {
            car.column >= 0
                && car.column <= WIDTH
                && car.row >= 0
                && car.row <= HEIGHT
        });
        
         // Effacer le canevas
         canvas.set_draw_color(Color::RGB(0, 0, 0));
         canvas.clear();
         matrix_and_canva(&mut canvas ,HEIGHT, WIDTH );
 
         // Dessiner chaque voitures
         canvas.set_draw_color(Color::RGB(255, 0, 0));
         for car in &cars {
             car.draw(&mut canvas);
         }
 
         // Mettre à jour le canevas
         canvas.present();
 
         // Limiter la boucle à environ 60 FPS
         std::thread::sleep(Duration::from_millis(16));
     }
}