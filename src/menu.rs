use adw::gio::{Menu, MenuModel};
use adw::prelude::Cast;
use gtk::PopoverMenuBar;

pub fn build_menu() -> PopoverMenuBar {
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

    popover_bar
}

