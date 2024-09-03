extern crate sdl2;
use sdl2::image::InitFlag;
use sdl2::pixels::Color;
use smart_road::cars::{detect_collisions, handle_collisions, update_cars, Car, Destinations};
use smart_road::matrix::{matrix_and_canva, COLUMN, ROW};
use smart_road::utils::random_spawn;
use std::time::{Duration, Instant};

const WIDTH: i32 = 800;
const HEIGHT: i32 = 800;
const SQUARE_SPEED: i32 = 2;
const ACTION_INTERVAL: Duration = Duration::from_millis(500);

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
    matrix_and_canva(&mut canvas, HEIGHT, WIDTH);
    canvas.present();
    let cell_size_width = WIDTH / COLUMN;
    let cell_size_height = HEIGHT / ROW;
    let cell_size = cell_size_width.min(cell_size_height);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut cars: Vec<Car> = Vec::new();
    let mut last_action_time = Instant::now();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'running,
                sdl2::event::Event::KeyDown { keycode, .. } => {
                    if last_action_time.elapsed() >= ACTION_INTERVAL {
                        match keycode {
                            Some(sdl2::keyboard::Keycode::Escape) => break 'running,
                            Some(sdl2::keyboard::Keycode::Down) => {
                                smart_road::utils::random_cars(
                                    Destinations::North,
                                    &texture_creator,
                                    SQUARE_SPEED,
                                    cell_size,
                                    &mut cars,
                                );
                            }
                            Some(sdl2::keyboard::Keycode::Up) => {
                                smart_road::utils::random_cars(
                                    Destinations::South,
                                    &texture_creator,
                                    SQUARE_SPEED,
                                    cell_size,
                                    &mut cars,
                                );
                            }
                            Some(sdl2::keyboard::Keycode::Left) => {
                                smart_road::utils::random_cars(
                                    Destinations::East,
                                    &texture_creator,
                                    SQUARE_SPEED,
                                    cell_size,
                                    &mut cars,
                                );
                            }
                            Some(sdl2::keyboard::Keycode::Right) => {
                                smart_road::utils::random_cars(
                                    Destinations::West,
                                    &texture_creator,
                                    SQUARE_SPEED,
                                    cell_size,
                                    &mut cars,
                                );
                            }
                            Some(sdl2::keyboard::Keycode::R) => {
                                smart_road::utils::random_cars(
                                    random_spawn(),
                                    &texture_creator,
                                    SQUARE_SPEED,
                                    cell_size,
                                    &mut cars,
                                );
                            }
                            _ => {}
                        }
                        // println!("One second has passed. Performing periodic action.");

                        // Update the last action time
                        last_action_time = Instant::now();
                    }
                }
                _ => {}
            }
        }

        let collisions = detect_collisions(&mut cars);
        handle_collisions(&mut cars, collisions);
        update_cars(&mut cars);

        cars.retain(|car| {
            car.column >= 0 && car.column <= WIDTH && car.row >= 0 && car.row <= HEIGHT
        });

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        matrix_and_canva(&mut canvas, HEIGHT, WIDTH);

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        for car in &cars {
            car.draw(&mut canvas);
        }

        canvas.present();
        std::thread::sleep(Duration::from_millis(16));
    }
}
