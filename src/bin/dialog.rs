use adw::glib::{ExitCode};
use adw::prelude::*;
use adw::{AlertDialog, Application};
use gtk::glib::clone;
use common::util;

fn main() -> ExitCode {
    let app = Application::builder()
        .application_id("hello.dialog")
        .build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {

    let btn_content = adw::ButtonContent::builder()
        .icon_name("document-open")
        .label("Open Project")
        .build();
    let btn = gtk::Button::builder().child(&btn_content).build();
    let window = util::layout_win(app, &btn);

    btn.connect_clicked(clone!(
        #[strong]
        window,
        move |_| {
            let alert = AlertDialog::builder()
                .title("Open Project")
                .body("Open the selected project in a new window or replace current window?")
                .build();

            alert.present(Some(&window));
        }
    ));

    window.set_content(Some(&btn));

    window.present();
}
