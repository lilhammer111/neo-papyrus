use adw::glib::ExitCode;
use adw::prelude::*;
use adw::Application;
use gtk::prelude::*;
use gtk::BoxLayout;
use gtk::Orientation::Horizontal;

fn main() -> ExitCode {
    let app = Application::builder()
        .application_id("hello.layout")
        .build();

    app.connect_activate(build_ui);

    app.run()
}

// BoxLayout make no sense ?
fn build_ui(app: &Application) {
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("Hello Layout")
        .height_request(600)
        .width_request(800)
        .build();
    let bl = BoxLayout::builder().orientation(Horizontal).build();

    let cont = gtk::Box::builder()
        .orientation(Horizontal)
        .layout_manager(&bl)
        .build();

    let b1 = gtk::Button::with_label("Click 1");
    let b2 = gtk::Button::with_label("Click 2");
    let b3 = gtk::Button::with_label("Click 3");

    cont.append(&b1);
    cont.append(&b2);
    cont.append(&b3);

    window.set_child(Some(&cont));
    window.present();
}
