use adw::glib::ExitCode;
use adw::prelude::*;
use common::fixed_win;
use gtk::Application;

fn main() -> ExitCode {
    let app = Application::builder()
        .application_id("hello.editor")
        .build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let fixed = gtk::Fixed::new();

    let btn = gtk::Button::builder().icon_name("document-open").build();

    fixed.put(&btn, 10f64, 1000f64);

    let win = fixed_win(app, &fixed);

    win.present();
}
