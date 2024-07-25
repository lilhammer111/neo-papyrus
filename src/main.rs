use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, PopoverMenuBar, Orientation, Box, Stack, StackSidebar, Label};
use gtk::gio::{Menu, MenuItem, MenuModel};

const APP_ID: &str = "org.gtk_rs.ListWidgets1";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn create_menu_model(labels: Vec<&str>, actions: Vec<&str>) -> MenuModel {
    let menu = Menu::new();

    let label_action: Vec<(&str, &str)> = labels.into_iter().zip(actions.into_iter()).collect();
    let submenu = Menu::new();
    for item in label_action.into_iter() {
        submenu.append(Some(item.0), Some(item.1));
    }

    menu.append_submenu(Some("SubMenu"), &submenu);
    menu.upcast()
}

fn create_menubar() -> PopoverMenuBar {
    let menu = Menu::new();

    let file_item = MenuItem::new(Some("File"), None);
    // let file_submenu_labels = vec!["Open", "New"];
    // let file_submenu_actions = vec!["",""];
    // let file_menu_model = create_menu_model(file_submenu_labels,file_submenu_actions);

    let edit_item = MenuItem::new(Some("Edit"), None);

    menu.append_item(&file_item);
    menu.append_item(&edit_item);
    let mm = menu.upcast::<MenuModel>();
    let popover = PopoverMenuBar::builder().hexpand(true).menu_model(&mm).build();
    popover
}


fn paint_pop_menu(menu: &MenuModel) -> PopoverMenuBar {
    PopoverMenuBar::builder()
        .menu_model(menu)
        .build()
}


fn create_layout() -> Box {
    let stack = Stack::builder().hexpand(true).build();

    // 创建不同的页面或组件，并添加到 Stack 中
    let page1 = Box::new(Orientation::Vertical, 5);
    page1.append(&Label::new(Some("This is Page 1")));

    let page2 = Box::new(Orientation::Vertical, 5);
    page2.append(&Label::new(Some("This is Page 2")));

    let page3 = Box::new(Orientation::Vertical, 5);
    page3.append(&Label::new(Some("This is Page 3")));

    stack.add_titled(&page1, Some("explore"), "Page 1");
    stack.add_titled(&page2, Some("page2"), "Page 2");
    stack.add_titled(&page3, Some("page3"), "Page 3");

    let stack_sidebar = StackSidebar::builder().width_request(60).build();
    stack_sidebar.set_stack(&stack);

    // let hbox = Box::new(Orientation::Horizontal, 5);
    let hbox = Box::builder().orientation(Orientation::Horizontal).vexpand(true).build();

    let vbox = Box::builder().orientation(Orientation::Vertical).hexpand(true).build();

    let popover_menubar = create_menubar();
    vbox.append(&popover_menubar);
    vbox.append(&stack);

    hbox.append(&stack_sidebar);
    hbox.append(&vbox);

    hbox
}

fn build_ui(app: &Application) {
    // let p_menubar = paint();
    let layout = create_layout();
    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .maximized(true)
        .title("Neo Papyrus")
        .width_request(1024)
        .height_request(768)
        .child(&layout)
        .build();

    // Present window
    window.present();
}
