use gtk::{Application, gio};
use adw::gio::{Menu, MenuModel};
use adw::prelude::{ActionMapExt, Cast};
use gtk::PopoverMenuBar;

pub fn build_menu(app: &Application) -> PopoverMenuBar {
    let file_menu = Menu::new();
    file_menu.append(Some("New File"), Some("app.file.new"));
    file_menu.append(Some("Open File"), Some("app.file.openf"));
    file_menu.append(Some("Open Folder"), Some("app.file.openfd"));
    file_menu.append(Some("Save File"), Some("app.file.save"));

    let help_menu = Menu::new();
    help_menu.append(Some("About"), Some("app.help.about"));

    let menu = Menu::new();
    menu.append_submenu(Some("File"), &file_menu);
    menu.append_submenu(Some("Help"), &help_menu);
    let mm = menu.upcast::<MenuModel>();

    let popover_bar = PopoverMenuBar::from_model(Some(&mm));

    create_actions(app);

    popover_bar
}

fn create_actions(app: &Application) {
    // 添加动作
    let action_new_file = gio::SimpleAction::new("file.new", None);
    action_new_file.connect_activate(|_, _| {
        println!("New File activated");
    });
    app.add_action(&action_new_file);

    let action_open_file = gio::SimpleAction::new("file.open", None);
    action_open_file.connect_activate(|_, _| {
        println!("Open File activated");
    });
    app.add_action(&action_open_file);

    let action_save_file = gio::SimpleAction::new("file.save", None);
    action_save_file.connect_activate(|_, _| {
        println!("Save File activated");
    });
    app.add_action(&action_save_file);

    let action_about = gio::SimpleAction::new("help.about", None);
    action_about.connect_activate(|_, _| {
        println!("About activated");
    });
    app.add_action(&action_about);
}