use gtk::Application;
use gtk::prelude::GtkApplicationExt;

pub fn setup_shortcuts(app: &Application) {
    app.set_accels_for_action("win.filter('All')", &["<Ctrl>a"]);
    app.set_accels_for_action("win.filter('Open')", &["<Ctrl>o"]);
    app.set_accels_for_action("win.filter('Done')", &["<Ctrl>d"]);

    app.set_accels_for_action("file.new", &["<Ctrl>n"]);
    app.set_accels_for_action("file.open", &["<Ctrl>o"]);
    app.set_accels_for_action("file.save", &["<Ctrl>s"]);
    app.set_accels_for_action("help.about", &["<Ctrl>a"]);
}