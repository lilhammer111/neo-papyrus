use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Image};

fn main() {
    let app = Application::new(Some("com.example.gtktest"), Default::default());
    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title(Option::from("Simple GTK App"));
        window.set_default_size(350, 70);

        // Replace "icon-name" with the actual icon name or path to an SVG file on your system
        let image = Image::from_file("/usr/share/icons/Adwaita/scalable/actions/list-add-symbolic.svg");
        window.set_child(Some(&image));

        window.present();
    });

    app.run();
}
