use adw::prelude::*;
use adw::Application;
use adw::NavigationPage;
use adw::NavigationSplitView;
use gtk::glib::ExitCode;
use gtk::PopoverMenuBar;

const APP_ID: &str = "org.gtk_rs.ListWidgets1";

fn main() -> ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_startup(setup_shortcuts);
    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn setup_shortcuts(app: &adw::Application) {
    app.set_accels_for_action("win.filter('All')", &["<Ctrl>a"]);
    app.set_accels_for_action("win.filter('Open')", &["<Ctrl>o"]);
    app.set_accels_for_action("win.filter('Done')", &["<Ctrl>d"]);
}

fn build_ui(app: &Application) {
    let sidebar = NavigationPage::builder().build();
    let content = NavigationPage::builder().build();

    let menubar = PopoverMenuBar

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
        .maximized(true)
        .child(&view)
        .build();

    window.present();
}
