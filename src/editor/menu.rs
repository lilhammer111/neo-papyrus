use adw::gio::{ActionEntry, Menu, MenuModel, SimpleActionGroup};
use adw::prelude::{ActionMapExtManual, Cast, FileEnumeratorExt, FileExt, ObjectExt};
use gtk::prelude::{DialogExt, FileChooserExt, GtkWindowExt, WidgetExt};
use gtk::{gio, ApplicationWindow, FileChooserAction, PopoverMenuBar, ResponseType};

pub fn build_menu(win: &ApplicationWindow) -> PopoverMenuBar {
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

    let file_actions_group = create_actions(win);

    win.insert_action_group("file", Some(&file_actions_group));

    popover_bar
}

fn create_actions(win: &ApplicationWindow) -> SimpleActionGroup {
    let action_new_proj = ActionEntry::builder("newp")
        .activate(move |_, _, _| println!("new project"))
        .build();

    let win = win.clone();
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

            dialog.connect_response(move |dialog, rkind| {
                if rkind == ResponseType::Accept {
                    if let Some(folder) = dialog.file() {
                        println!("Selected folder: {:?}", folder);
                        let file_enumerator = folder
                            .enumerate_children(
                                "*",
                                gio::FileQueryInfoFlags::NONE,
                                gio::Cancellable::NONE,
                            )
                            .expect("failed to enumerate folder's children");

                        while let Some(info) = file_enumerator
                            .next_file(gio::Cancellable::NONE)
                            .expect("Failed to get next file")
                        {
                            // 在这里构建树状结构
                            // let child_file = file_enumerator.child(&info);
                            // let file_name = child_file.name().expect("Failed to get file name");
                            println!("Info: {:?}", info);
                            dialog.emit_by_name("folder-selected", &[&folder]);
                        }
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
