use adw::glib::ExitCode;
use adw::prelude::{ApplicationExt, ApplicationExtManual};
use adw::Application;
use gtk::prelude::GtkWindowExt;
use gtk::ApplicationWindow;
use std::env;
use std::ops::Add;

const ICON_PATH: &str = "/usr/share/icons/Adwaita/symbolic";
const VAR_XDG_DIRS: &str = "XDG_DATA_DIRS";
const VAR_GTK_THEME:&str = "GTK_THEME";

fn main() -> ExitCode {
    env::set_var(VAR_GTK_THEME, "Adwaita");

    let xdg=format!("{}:", ICON_PATH).add(VAR_XDG_DIRS);
    env::set_var(VAR_XDG_DIRS, xdg);

    println!("{:?}",env::var(VAR_XDG_DIRS));

    let app = Application::builder().application_id("icon.test").build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let btn = gtk::Button::builder()
        .label("Open")
        .icon_name("document-open-symbolic")
        .vexpand(false)
        .hexpand(false)
        .build();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("hello")
        .default_width(800)
        .default_height(600)
        .child(&btn)
        .build();

    window.present();
}
