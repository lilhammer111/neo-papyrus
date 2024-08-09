mod menu;
mod shortcuts;

use crate::menu::build_menu;
use adw::glib::ExitCode;
use adw::prelude::*;
use gtk::Application;
use gtk::Orientation::Vertical;
use crate::shortcuts::setup_shortcuts;

fn main() -> ExitCode {
    let app = Application::builder()
        .application_id("hello.editor")
        .build();
    app.connect_activate(build_ui);
    app.connect_startup(setup_shortcuts);
    app.run()
}

fn build_ui(app: &Application) {
    let win = adw::ApplicationWindow::builder()
        .title("Editor")
        .default_width(1000)
        .default_height(780)
        .application(app)
        .build();

    let header_bar = adw::HeaderBar::builder().build();

    let menu = build_menu(&win);

    let mbox = gtk::Box::builder().orientation(Vertical).build();

    mbox.append(&header_bar);
    mbox.append(&menu);

    win.set_content(Some(&mbox));
    win.present();
}
