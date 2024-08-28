extern crate sdl2;
use smart_road::matrix::matrix_and_canva; // Import the draw_matrix function
use sdl2::image::InitFlag;
use sdl2::pixels::Color;
// use crate::sdl2::image::LoadTexture;

const WIDTH: u32 = 800; // Example width
const HEIGHT: u32 = 800; // Example height

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();


    sdl2::image::init(InitFlag::PNG | InitFlag::JPG | InitFlag::WEBP).unwrap();

    let window = video_subsystem
        .window("SDL2 Window", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    // let texture_creator = canvas.texture_creator();

    // let background_texture = texture_creator.load_texture("./src/images/roads.png").unwrap();


    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    
    
    //Permet d'implementer la matrix dans le canva
    // canvas.copy(&background_texture, None, None).unwrap();
    matrix_and_canva(&mut canvas ,HEIGHT, WIDTH );
    
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
