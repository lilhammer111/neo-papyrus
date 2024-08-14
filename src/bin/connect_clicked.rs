use gtk::prelude::*;
use gtk::Application;

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id("hello").build();

    // Connect to "activate" signal of app
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(application: &Application) {
    // Create two buttons
    let button_increase = gtk::Button::builder()
        .label("Increase")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // A mutable integer
    let mut number = 0;

    // Connect callbacks
    // When a button is clicked, number should be changed
    button_increase.connect_clicked(move |_| number += 1);
    // error[E0594]: cannot assign to `number`, as it is a captured variable in a `Fn` closure
    //   --> src/bin/connect_clicked.rs:30:46
    //    |
    // 30 |     button_increase.connect_clicked(move |_| number += 1);
    //    |                                              ^^^^^^^^^^^ cannot assign

    // 因为这里connect_clicked的闭包类型约束为Fn，
    // 而，number += 1使得闭包move |_| number += 1的类型为FnMut，不符合入参类型约束

    // Create a window
    let window = gtk::ApplicationWindow::builder()
        .application(application)
        .title("My GTK App")
        .child(&button_increase)
        .build();

    // Present the window
    window.present();
}

