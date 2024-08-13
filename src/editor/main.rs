mod action;
mod menu;
mod shortcuts;
mod util;
mod view;

use crate::menu::build_menu;
use crate::shortcuts::setup_shortcuts;
use crate::view::build_view;
use adw::glib::ExitCode;
use adw::prelude::*;
use adw::{self, gdk};
use gtk::gio;
use gtk::Align::{End, Start};
use gtk::Orientation::Vertical;
fn main() -> ExitCode {
    let app = adw::Application::builder()
        .application_id("hello.editor")
        .build();
    app.connect_activate(build_ui);
    app.connect_startup(setup_shortcuts);
    app.connect_startup(load_css);
    app.run()
}

fn load_css(_: &adw::Application) {
    // 创建和加载 CSS Provider
    let provider = gtk::CssProvider::new();
    let gfile = gio::File::for_path("src/editor/index.css");
    provider.load_from_file(&gfile);
    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Error initializing GTK display"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    )
}

fn build_ui(app: &adw::Application) {
    let win = adw::ApplicationWindow::builder()
        .title("Editor")
        .default_width(1000)
        .default_height(800)
        .maximized(true)
        .application(app)
        .build();

    let mbox = gtk::Box::builder().orientation(Vertical).build();

    let (view, expander, text_buffer) = build_view(&win);
    let header_bar = adw::HeaderBar::builder().build();
    mbox.append(&header_bar);

    let menu = build_menu(&win, &expander, &text_buffer);
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

    win.set_content(Some(&overlay));
    win.show();
}
