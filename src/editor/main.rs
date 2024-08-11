mod menu;
mod shortcuts;
mod view;

use crate::menu::build_menu;
use crate::shortcuts::setup_shortcuts;
use crate::view::build_view;
use adw::glib::ExitCode;
use adw::prelude::*;
use gtk::Align::{End, Start};
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

fn build_ui(app: &Application) {
    let win = gtk::ApplicationWindow::builder()
        .title("Editor")
        .default_width(1000)
        .default_height(780)
        .application(app)
        .build();

    // let header_bar = adw::HeaderBar::builder().build();
    let menu = build_menu(&win);

    let mbox = gtk::Box::builder().orientation(Vertical).build();

    let view = build_view(&win);
    // mbox.append(&header_bar);
    mbox.append(&menu);
    mbox.append(&view);

    let overlay = gtk::Overlay::builder().child(&mbox).build();

    let btn = gtk::Button::builder()
        .has_frame(false)
        .icon_name("list-add-symbolic")
        .halign(Start)
        .valign(End)
        .margin_bottom(10)
        .margin_start(10)
        .build();
    overlay.add_overlay(&btn);


    win.set_child(Some(&overlay));
    win.present();
}
