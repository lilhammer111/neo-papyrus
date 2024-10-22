mod action;
mod menu;
mod shortcuts;
mod view;
mod core;

use crate::menu::build_menu;
use crate::shortcuts::setup_shortcuts;
use crate::view::build_view;
use adw::glib::ExitCode;
use adw::prelude::*;
use adw::{self, gdk};
use gtk::gio;
use gtk::Align::{End, Start};
use gtk::Orientation::Vertical;

const APP_ID: &str = "wang.hammer.editor";
fn main() -> ExitCode {
    let app = adw::Application::builder().application_id(APP_ID).build();
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
    let header_bar = adw::HeaderBar::new();
    mbox.append(&header_bar);


    let (view, scrl_window, tabview) = build_view(&win);
    let menu = build_menu(&win, &scrl_window, &tabview);
    mbox.append(&menu);
    mbox.append(&view);

    let overlay = gtk::Overlay::builder().child(&mbox).build();

    let file_add_btn = gtk::Button::builder()
        .has_frame(false)
        .icon_name("list-add-symbolic")
        .halign(Start)
        .valign(End)
        .margin_bottom(10)
        .margin_start(10)
        .build();
    overlay.add_overlay(&file_add_btn);

    win.set_content(Some(&overlay));
    win.show();
}
