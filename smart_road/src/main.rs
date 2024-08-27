extern crate sdl2;

use sdl2::image::InitFlag;
use sdl2::pixels::Color;
use smart_road::utils::sprite_map;

const WIDTH: u32 = 800; // Example width
const HEIGHT: u32 = 800; // Example height

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    
    // Initialize image subsystem
    sdl2::image::init(InitFlag::PNG | InitFlag::JPG | InitFlag::WEBP).unwrap();
    
    let window = video_subsystem
        .window("SDL2 Window", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();
    
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    
    // Clear the canvas with a color
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    
    // Call your sprite generation function
    sprite_map(&mut canvas, &texture_creator, HEIGHT, WIDTH);
    
    canvas.present();
    
    // Main loop placeholder
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'running,
                _ => {}
            }
        }
    }
}
