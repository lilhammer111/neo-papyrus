use adw::glib::{clone, ExitCode};
use adw::prelude::{ApplicationExt, ApplicationExtManual, FileExt};
use adw::{gio, glib, Application, Window};
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt};
use gtk::Orientation::{Horizontal, Vertical};
use gtk::{FileDialog, Label, PolicyType, ScrolledWindow};
use std::fs::read_to_string;

fn main() -> ExitCode {
    let app = Application::builder()
        .application_id("com.exa.FileDialog")
        .build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("Test File Dialog")
        .default_height(800)
        .default_width(1_000)
        .build();

    let vbox = gtk::Box::new(Vertical, 0);
    window.set_child(Some(&vbox));

    let button = gtk::Button::builder()
        // .label("Click me")
        .icon_name("document-open")
        .hexpand(false)
        .vexpand(false)
        .build();

    let hbox = gtk::Box::builder().orientation(Horizontal).build();
    hbox.append(&button);
    vbox.append(&hbox);

    // create a scroll window
    let scrl_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .vscrollbar_policy(PolicyType::Automatic)
        .hexpand(true)
        .vexpand(true)
        .build();
    vbox.append(&scrl_window);

    let label = Label::builder().wrap(true).build();
    scrl_window.set_child(Some(&label));

    button.connect_clicked(clone!(
        #[strong]
        label,
        move |_| {
            let dialog = FileDialog::builder()
                .title("Choose a File")
                .modal(true)
                .build();

            let label = label.clone();
            let cls = move |result: Result<gio::File, glib::Error>| match result {
                Ok(file) => {
                    println!("File: {:?}", file.path().expect("failed to get path"));

                    if let Some(path) = file.path() {
                        match read_to_string(path) {
                            Ok(contents) => label.set_text(&contents),
                            Err(error) => {
                                let err_tooltip = format!("Error: {:?}", error);
                                label.set_text(&err_tooltip)
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("Err: {:?}", e)
                }
            };

            dialog.open(None::<&Window>, None::<&gio::Cancellable>, cls);
        }
    ));

    window.present();
}
