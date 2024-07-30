use adw::{Application, NavigationPage, NavigationSplitView};
use adw::gio::{Menu, MenuModel};
use adw::glib::ExitCode;
use adw::prelude::{ApplicationExt, ApplicationExtManual, Cast};
use gtk::{ApplicationWindow, Label, MenuButton, PopoverMenu};
use gtk::prelude::GtkWindowExt;

const APP_ID: &str = "org.gtk_rs.MenuBtn";


fn main() -> ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();


    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}


fn build_ui(app: &Application) {

    let menu = Menu::new();
    menu.append(Some("HELLO"), None);

    let menu_model = menu.upcast::<MenuModel>();
    let popover = PopoverMenu::from_model(Some(&menu_model));

    let menu_btn =MenuButton::builder()
        .label("click me")
        .popover(&popover)
        .build();
    let sidebar = NavigationPage::new(&menu_btn, "Sidebar");

    let label = Label::new(Some("Hello world"));
    let content = NavigationPage::new(&label, "Content");

    let sview = NavigationSplitView::builder()
        .sidebar(&sidebar)
        .content(&content)
        .build();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Hello GTK")
        .child(&sview)
        .height_request(600)
        .width_request(800)
        .build();

    window.present();
}