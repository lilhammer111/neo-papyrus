use crate::action::file_actions;
use adw::gio::{Menu, MenuModel};
use adw::prelude::Cast;
use gtk::prelude::WidgetExt;
use gtk::{PopoverMenuBar};

pub fn build_menu(
    win: &adw::ApplicationWindow,
    scrl_window: &gtk::ScrolledWindow,
    tabview: &adw::TabView,
) -> PopoverMenuBar {
    let file_menu = Menu::new();
    file_menu.append(Some("New Project"), Some("file.newp"));
    file_menu.append(Some("Open Project"), Some("file.openp"));
    file_menu.append(Some("New File"), Some("file.newf"));
    file_menu.append(Some("New File"), Some("file.openf"));
    let help_menu = Menu::new();
    help_menu.append(Some("About"), Some("about"));

    let menu = Menu::new();
    menu.append_submenu(Some("File"), &file_menu);
    menu.append_submenu(Some("Help"), &help_menu);
    let mm = menu.upcast::<MenuModel>();

    let popover_bar = PopoverMenuBar::from_model(Some(&mm));

    // create file actions handler
    let file_actions_group = file_actions(win, scrl_window, tabview);

    win.insert_action_group("file", Some(&file_actions_group));
    popover_bar
}
