use adw::prelude::*;
use adw::{Application, NavigationPage, NavigationSplitView};
use gtk::glib::ExitCode;
use gtk::Button;
use gtk::Orientation::Vertical;

fn main() -> ExitCode {
    let app = Application::builder().application_id("hello.nsv").build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let vbox = gtk::Box::builder()
        .orientation(Vertical)
        .hexpand(true)
        .margin_top(6)
        .margin_bottom(6)
        .build();


    let btn = Button::builder()
        .icon_name("document-open-symbolic")
        .build();

    let sidebar_box = gtk::Box::builder().orientation(Vertical).spacing(10).build();
    sidebar_box.append(&btn);

    let sidebar = NavigationPage::builder().child(&sidebar_box).build();
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
