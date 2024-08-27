pub use gtk::prelude::*;
pub use gtk::{Overlay, Image, Application, ApplicationWindow, Fixed};
pub use crate::matrice::*;
#[macro_use]
extern crate gtk;
pub fn interface(length: u32, width: u32) {
    let app = Application::new(
        Some("com.example.mon_interface"),
        Default::default(),
    );

    app.connect_activate(move |app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Smart Road");
        window.set_default_size(length as i32, width as i32);

        let overlay = Overlay::new();
        let image = Image::from_file("../smart_road/src/images/routes.png");

        // Crée un conteneur Fixed pour positionner les widgets
        let fixed_container = Fixed::new();

        // Ajoute l'image en fond d'écran
        overlay.add(&image);

        // Ajoute le conteneur pour les autres widgets en superposition
        overlay.add_overlay(&fixed_container);

        // Redimensionner l'image en fonction de la taille de la fenêtre
        window.connect_configure_event(clone!(@weak image => move |window, _| {
            let (new_width, new_height) = window.get_size();
            image.set_size_request(new_width, new_height);
            image.set_from_pixbuf(
                &image.get_pixbuf().unwrap().scale_simple(
                    new_width, 
                    new_height, 
                    gdk_pixbuf::InterpType::Bilinear
                ).unwrap()
            );
            Inhibit(false)
        }));

        // Ajoute l'Overlay à la fenêtre
        window.add(&overlay);

        window.show_all();
    });

    app.run();
}
