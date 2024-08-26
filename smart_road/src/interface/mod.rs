pub use gtk::prelude::*;
pub use gtk::{Grid, Image, Application, ApplicationWindow};
pub use crate::matrice::*;

pub fn interface(length: u32, width: u32) {
    let app = Application::new(
        Some("com.example.mon_interface"),
        Default::default(),
    );

    app.connect_activate(move |app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Smart Road");
        window.set_default_size((length) as i32, (width) as i32);
        let grid = Grid::new();
        let image = Image::from_file("../smart_road/src/images/route.png");
        grid.attach(&image, 0, 0, 1, 1);  // Assume que tu utilises une grille pour organiser tes widgets

        window.show_all();
    });

    app.run();
}
