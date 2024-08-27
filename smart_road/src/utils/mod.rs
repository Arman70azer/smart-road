use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub fn applicate_texture(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>, filename: String, heigth: u32, width: u32) {
    // Load a texture from an image file
    let texture = texture_creator.load_texture(filename).unwrap();

    // Define the destination rectangle (where on the canvas to draw the texture)
    let dest_rect = Rect::new(0, 0, width, heigth); // Example position and size

    // Clear the canvas to white before drawing
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();

    // Copy the texture to the canvas
    canvas.copy(&texture, None, Some(dest_rect)).unwrap();
}