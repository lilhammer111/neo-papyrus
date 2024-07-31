mod menu;

use crate::menu::build_menu;
use adw::glib::ExitCode;
use adw::prelude::*;
use gtk::Application;
use gtk::Orientation::Vertical;

fn main() -> ExitCode {
    let app = Application::builder()
        .application_id("hello.editor")
        .build();
    app.connect_activate(build_ui);
    app.connect_startup(setup_shortcuts);
    app.run()
}

fn setup_shortcuts(app: &Application) {
    app.set_accels_for_action("win.filter('All')", &["<Ctrl>a"]);
    app.set_accels_for_action("win.filter('Open')", &["<Ctrl>o"]);
    app.set_accels_for_action("win.filter('Done')", &["<Ctrl>d"]);

    app.set_accels_for_action("app.file.new", &["<Primary>n"]);
    app.set_accels_for_action("app.file.open", &["<Primary>o"]);
    app.set_accels_for_action("app.file.save", &["<Primary>s"]);
    app.set_accels_for_action("app.help.about", &["<Primary>a"]);
}

fn build_ui(app: &Application) {
    let header_bar = adw::HeaderBar::builder().build();

    let menu = build_menu(app);

    let mbox = gtk::Box::builder().orientation(Vertical).build();

    mbox.append(&header_bar);
    mbox.append(&menu);

    adw::ApplicationWindow::builder()
        .title("Editor")
        .default_width(1000)
        .default_height(780)
        .application(app)
        .content(&mbox)
        .build()
        .present();
}
