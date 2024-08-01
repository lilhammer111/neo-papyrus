use adw::gio::{ActionEntry, Menu, MenuModel, SimpleActionGroup};
use adw::prelude::{ActionMapExtManual, Cast};
use adw::ApplicationWindow;
use gtk::prelude::WidgetExt;
use gtk::PopoverMenuBar;

pub fn build_menu(win: &ApplicationWindow) -> PopoverMenuBar {
    let file_menu = Menu::new();
    file_menu.append(Some("New Project"), Some("file.new"));
    file_menu.append(Some("Open Project"), Some("file.open"));

    let help_menu = Menu::new();
    help_menu.append(Some("About"), Some("about"));

    let menu = Menu::new();
    menu.append_submenu(Some("File"), &file_menu);
    menu.append_submenu(Some("Help"), &help_menu);
    let mm = menu.upcast::<MenuModel>();

    let popover_bar = PopoverMenuBar::from_model(Some(&mm));

    let file_actions_group = create_actions();
    win.insert_action_group("file", Some(&file_actions_group));

    popover_bar
}

fn create_actions() -> SimpleActionGroup {
    let action_new_proj = ActionEntry::builder("new")
        .activate(move |_, _, _| println!("New Project"))
        .build();

    let action_open_proj = ActionEntry::builder("open")
        .activate(move |_, _, _| println!("open project"))
        .build();

    let file_actions = SimpleActionGroup::new();

    file_actions.add_action_entries([action_new_proj, action_open_proj]);

    file_actions
}
