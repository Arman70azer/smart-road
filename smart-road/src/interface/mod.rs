pub use gtk::prelude::*;
pub use gtk::{Application, ApplicationWindow, Button};

pub fn interface(length: u8, width: u8) {
    let app = Application::new(
        Some("com.example.mon_interface"),
        Default::default(),
    );

    app.connect_activate(move |app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Mon Interface Rust");
        window.set_default_size((length * 10) as i32, (width * 10) as i32);

        let button = Button::with_label("Clique moi!");
        button.connect_clicked(|_| {
            println!("Bouton cliqué!");
        });

        window.add(&button);
        window.show_all();
    });

    app.run();
}
