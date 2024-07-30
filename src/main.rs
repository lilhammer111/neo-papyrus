mod action;
mod menu;
mod layout;

use crate::action::setup_shortcuts;
use crate::menu::build_menu;
use adw::prelude::*;
use adw::Application;
use adw::NavigationPage;
use adw::NavigationSplitView;
use gtk::glib::ExitCode;
use gtk::Orientation::Vertical;

fn main() -> ExitCode {
    // Create a new application
    let app = Application::builder()
        .application_id("org.gtk_rs.NeoPapyrus")
        .build();
    app.connect_startup(setup_shortcuts);
    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}
fn build_ui(app: &Application) {
    let vbox = gtk::Box::builder()
        .orientation(Vertical)
        .hexpand(true)
        // .valign(Align::Center)
        .margin_top(6)
        .margin_bottom(6)
        .build();

    let popover_bar = build_menu();

    vbox.append(&popover_bar);

    let sidebar = NavigationPage::builder().build();
    let content = NavigationPage::builder().child(&vbox).build();

    let view = NavigationSplitView::builder()
        .content(&content)
        .sidebar(&sidebar)
        .max_sidebar_width(60.0)
        .min_sidebar_width(60.0)
        .sidebar_width_unit(adw::LengthUnit::Px)
        .build();

    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("Neo Papyrus")
        .height_request(800)
        .width_request(1096)
        // .maximized(true)
        .child(&view)
        .build();

    window.present();
}
