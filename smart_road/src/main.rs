extern crate sdl2;
use smart_road::matrix::draw_matrix; // Import the draw_matrix function
use smart_road::matrix::Matrix;
use sdl2::image::InitFlag;
use sdl2::pixels::Color;

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
    //let texture_creator = canvas.texture_creator();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    // Example matrix to draw
    let matrix: Matrix = vec![
        vec![0, 1, 0, 1],
        vec![1, 0, 1, 0],
        vec![0, 1, 0, 1],
        vec![1, 0, 1, 0],
    ];

    let num_of_rows = matrix.len();
    let num_of_cols = if num_of_rows > 0 { matrix[0].len() } else { 0 };

    // Ensure non-zero dimensions
    if num_of_rows == 0 || num_of_cols == 0 {
        println!("Matrix is empty.");
        return;
    }

    // Calculate cell size based on canvas dimensions
    let cell_size_width = WIDTH / num_of_cols as u32;
    let cell_size_height = HEIGHT / num_of_rows as u32;

    // Use the smaller of the two dimensions to ensure cells fit in the canvas
    let cell_size = cell_size_width.min(cell_size_height);

    draw_matrix(&mut canvas, &matrix, cell_size);

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
