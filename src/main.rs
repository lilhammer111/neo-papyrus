mod layout;

use adw::prelude::*;
use adw::Application;
use gtk::glib::ExitCode;
use crate::layout::View;

fn main() -> ExitCode {
    // Create a new application
    let app = Application::builder()
        .application_id("com.neo-papyrus")
        .build();
    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui() {

}
