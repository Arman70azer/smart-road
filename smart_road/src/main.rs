use std::time::{Duration, Instant};
use smart_road::matrix::{matrix_and_canva, ROW, COLUMN};
use smart_road::cars::sub_mod_cars::Cars;
use smart_road::statistics::{init_font, display_stats};
use sdl2::image::InitFlag;
use sdl2::pixels::Color;

// Importez la fonction handle_keydown depuis le module event_handler
mod event_handler;
use event_handler::handle_keydown;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 800;
const FRAME_DURATION: Duration = Duration::from_millis(16);

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

    let cell_size = (WIDTH / COLUMN).min(HEIGHT / ROW);
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut cars = Cars::new();
    let mut last_action_time = Instant::now();
    let ttf_context = sdl2::ttf::init().expect("Failed to initialize TTF context");
    let font = init_font(&ttf_context);
    let mut see_tab = false;

    'running: loop {
        for event in event_pump.poll_iter() {
            if let sdl2::event::Event::KeyDown { keycode: Some(keycode), .. } = event {
                handle_keydown(keycode, &mut see_tab, &mut last_action_time, &mut cars, &texture_creator, cell_size);
            }
            if let sdl2::event::Event::Quit { .. } = event {
                break 'running;
            }
        }

        cars.handle_collisions();
        cars.update_cars();
        cars.retain(HEIGHT, WIDTH);

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        matrix_and_canva(&mut canvas, HEIGHT, WIDTH);

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        for car in &cars.cars {
            car.draw(&mut canvas);
        }

        if see_tab && cars.cars.is_empty() {
            display_stats(&mut canvas, &font, &texture_creator, &cars);
        }

        canvas.present();
        std::thread::sleep(FRAME_DURATION);
    }
}
