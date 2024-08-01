use adw::prelude::*;
use gtk::glib::ExitCode;
use gtk::{Application, ScrolledWindow, WrapMode};
use gtk::PolicyType;

fn main() -> ExitCode {
    println!("GTK_IM_MODULE={}",env!("GTK_IM_MODULE"));
    println!("QT_IM_MODULE={}",env!("QT_IM_MODULE"));
    println!("XMODIFIERS={}",env!("XMODIFIERS"));
    let app = Application::builder()
        .application_id("hello.textview")
        .build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let win = gtk::ApplicationWindow::builder()
        .application(app)
        .title("Hello textview")
        .default_width(800)
        .width_request(800)
        .default_height(600)
        .height_request(600)
        .build();

    let tv = gtk::TextView::builder()
        .vexpand(true)
        .hexpand(true)
        .accepts_tab(true)
        .wrap_mode(WrapMode::WordChar)
        .build();

    let sw = ScrolledWindow::builder()
        .vscrollbar_policy(PolicyType::Automatic)
        .hscrollbar_policy(PolicyType::Never)
        .child(&tv)
        .build();

    win.set_child(Some(&sw));
    win.present()
}
