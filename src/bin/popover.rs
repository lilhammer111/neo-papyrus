use adw::prelude::*;
use adw::Application;
use gtk::glib::ExitCode;
use gtk::{gio, PopoverMenuBar};
use gtk::gio::MenuModel;
use gtk::Orientation::Vertical;

const APP_ID: &str = "popover_menu_bar";

fn main() -> ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_startup(setup_shortcuts);
    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn setup_shortcuts(app: &Application) {
    app.set_accels_for_action("win.filter('All')", &["<Ctrl>a"]);
    app.set_accels_for_action("win.filter('Open')", &["<Ctrl>o"]);
    app.set_accels_for_action("win.filter('Done')", &["<Ctrl>d"]);

    app.set_accels_for_action("file.new", &["<Ctrl>N"]);
    app.set_accels_for_action("file.open", &["<Ctrl>O"]);
    app.set_accels_for_action("file.save", &["<Ctrl>S"]);
}


fn build_ui(app: &Application) {
    let file_menu = gio::Menu::new();
    file_menu.append(Some("New"), Some("file.new"));
    file_menu.append(Some("Open"), Some("file.open"));
    file_menu.append(Some("Save"), Some("file.save"));

    let menu = gio::Menu::new();
    menu.append_submenu(Some("File"), &file_menu);

    let pmb = PopoverMenuBar::from_model(Some(&menu.upcast::<MenuModel>()));

    let window_box = gtk::Box::builder().orientation(Vertical).build();

    window_box.append(&pmb);

    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("Neo Papyrus")
        .height_request(800)
        .width_request(1096)
        .child(&window_box)
        .build();

    window.present();
}
