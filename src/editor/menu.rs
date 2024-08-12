use crate::util::render_directory;
use adw::gio::{ActionEntry, Menu, MenuModel, SimpleActionGroup};
use adw::prelude::{ActionMapExtManual, Cast, FileExt, PreferencesRowExt};
use gtk::prelude::{DialogExt, FileChooserExt, GtkWindowExt, WidgetExt};
use gtk::{FileChooserAction, PopoverMenuBar, ResponseType, TextBuffer};

pub fn build_menu(
    win: &adw::ApplicationWindow,
    expander: &adw::ExpanderRow,
    text_bf: &TextBuffer,
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

    let file_actions_group = create_actions(win, expander, text_bf);

    win.insert_action_group("file", Some(&file_actions_group));
    popover_bar
}

fn create_actions(
    win: &adw::ApplicationWindow,
    expander: &adw::ExpanderRow,
    text_bf: &TextBuffer,
) -> SimpleActionGroup {
    let action_new_proj = ActionEntry::builder("newp")
        .activate(move |_, _, _| println!("new project"))
        .build();

    let win = win.clone();
    let expander_cloned = expander.clone();
    let text_bf_cloned = text_bf.clone();
    let action_open_proj = ActionEntry::builder("openp")
        .activate(move |_, _, _| {
            let dialog = gtk::FileChooserDialog::builder()
                .title("open projects")
                .action(FileChooserAction::SelectFolder)
                .transient_for(&win)
                .modal(true)
                .build();

            dialog.add_button("Cancel", ResponseType::Cancel);
            dialog.add_button("Open", ResponseType::Accept);
            let expander_cloned = expander_cloned.clone();
            let text_bf_cloned = text_bf_cloned.clone();
            dialog.connect_response(move |dialog, rkind| {
                if rkind == ResponseType::Accept {
                    if let Some(gio_file) = dialog.file() {
                        let pb = gio_file.basename().unwrap();
                        let dirname = pb.file_name().unwrap().to_str().unwrap();
                        expander_cloned.set_title(dirname);
                        render_directory(&gio_file, &text_bf_cloned, &expander_cloned, 20);
                    }
                }
                dialog.close();
            });

            dialog.show();
        })
        .build();

    let action_new_file = ActionEntry::builder("newf")
        .activate(move |_, _, _| println!("new file"))
        .build();

    let action_open_file = ActionEntry::builder("openf")
        .activate(move |_, _, _| println!("open file"))
        .build();

    let file_actions = SimpleActionGroup::new();

    file_actions.add_action_entries([
        action_new_proj,
        action_open_proj,
        action_new_file,
        action_open_file,
    ]);

    file_actions
}
