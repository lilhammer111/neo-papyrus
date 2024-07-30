use adw::Application;
use gtk::prelude::GtkApplicationExt;

pub fn setup_shortcuts(app: &Application) {
    app.set_accels_for_action("win.filter('All')", &["<Ctrl>a"]);
    app.set_accels_for_action("win.filter('Open')", &["<Ctrl>o"]);
    app.set_accels_for_action("win.filter('Done')", &["<Ctrl>d"]);

    app.set_accels_for_action("app.file.new", &["<Ctrl>N"]);
    app.set_accels_for_action("app.file.open", &["<Ctrl>O"]);
    app.set_accels_for_action("app.file.save", &["<Ctrl>S"]);
}