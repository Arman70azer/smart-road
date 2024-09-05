use crate::cars::sub_mod_cars::Cars;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
//use sdl2::render::TextureCreator;
use sdl2::ttf::Font;

pub fn init_font<'a>(ttf_context: &'a sdl2::ttf::Sdl2TtfContext) -> sdl2::ttf::Font<'a, 'a> {
    let font_path = "src/font/LEMONMILK-Light.otf"; // Chemin vers votre fichier de police
    ttf_context.load_font(font_path, 24).expect("Failed to load font")
}

pub fn display_stats(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    font: &Font,
    texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    cars: &Cars,// Quand nous aurons la logique des level_speed
) {

    let stats = vec![
        format!("Statistics"),
        format!("Cars Passed: {}", cars.cars_passed),
        format!("Max velocity: {}px/s", cars.max_velocity),
        format!("Min velocity: {}px/s", cars.min_velocity),
        format!("Max Time: {:?}", cars.max_time),
        format!("Min Time: {:?}", cars.min_time),
        format!("Collisions: {}", cars.collisions),
        format!("Close Calls: {}", cars.close_calls),
    ];

    let line_height = 50; 
    let margin = 10; 
    let total_height = stats.len() * (line_height + margin); 
    let surface_width = 300; 

    let canvas_width = canvas.output_size().unwrap().0;
    let canvas_height = canvas.output_size().unwrap().1;

    let x = (canvas_width / 2) as i32 - (surface_width / 2) as i32;
    let y = (canvas_height / 2) as i32 - (total_height / 2) as i32;

    // Dessiner la surface noire directement sur le canevas
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.fill_rect(Rect::new(x-50, y-50, surface_width+100, (total_height+100)as u32))
        .expect("Failed to draw black rectangle");

    for (i, stat) in stats.iter().enumerate() {
        let text_surface = font.render(stat)
            .blended(Color::RGB(255, 255, 255))
            .expect("Failed to render text");

        let text_texture = texture_creator.create_texture_from_surface(&text_surface)
            .expect("Failed to create texture from surface");

        let y_position = i as i32 * (line_height + margin) as i32 + y;

        // Copier le texte sur le canevas
        canvas.copy(&text_texture, None, Some(Rect::new(x, y_position, surface_width, line_height as u32)))
            .expect("Failed to copy texture to canvas");
    }

    canvas.present();
}