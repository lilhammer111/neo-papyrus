use adw::prelude::*;
use adw::Application;
use adw::gio::SimpleAction;
use gtk::glib::ExitCode;
use gtk::{gio, PopoverMenuBar};
use gtk::gio::MenuModel;
use gtk::Orientation::Vertical;

const APP_ID: &str = "org.gtk_rs.Clickable";

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

    app.set_accels_for_action("app.file.new", &["<Ctrl>N"]);
    app.set_accels_for_action("app.file.open", &["<Ctrl>O"]);
    app.set_accels_for_action("app.file.save", &["<Ctrl>S"]);
}


fn build_ui(app: &Application) {

    let file_menu = gio::Menu::new();
    file_menu.append(Some("New"), Some("app.file.new"));
    file_menu.append(Some("Open"), Some("app.file.open"));
    file_menu.append(Some("Save"), Some("app.file.save"));

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

    // Add actions to the window
    let action_new = SimpleAction::new("file.new", None);
    action_new.connect_activate(|_, _| {
        println!("New File");
    });
    app.add_action(&action_new);  //app.add_action is key point

    let action_open = SimpleAction::new("file.open", None);
    action_open.connect_activate(|_, _| {
        println!("Open File");
    });
    app.add_action(&action_open);

    let action_save = SimpleAction::new("file.save", None);
    action_save.connect_activate(|_, _| {
        println!("Save File");
    });
    app.add_action(&action_save);


    window.present();
}
