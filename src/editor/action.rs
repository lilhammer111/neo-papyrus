use crate::util::{render_children_dir, INDENT_MARGIN};
use adw::gio;
use adw::gio::{ActionEntry, SimpleActionGroup};
use adw::prelude::{ActionMapExtManual, ExpanderRowExt, FileExt, PreferencesRowExt, SettingsExt};
use gtk::prelude::{DialogExt, FileChooserExt, GtkWindowExt, ListBoxRowExt, WidgetExt};
use gtk::{FileChooserAction, Label, ResponseType, TextBuffer};

use crate::APP_ID;

pub fn file_actions(
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
            dialog.connect_response(move |dialog, resp_kind| {
                if resp_kind == ResponseType::Accept {
                    if let Some(gio_file) = dialog.file() {
                        // new an empty child
                        // expander_cloned.set_child(None::<&Label>);
                        expander_cloned.remove(&expander_cloned.child().unwrap());

                        // set project name to expander
                        let fname_pb = gio_file.basename().unwrap();
                        let dirname = fname_pb.to_str().unwrap();
                        expander_cloned.set_title(dirname);

                        render_children_dir(
                            &gio_file,
                            &text_bf_cloned,
                            &expander_cloned,
                            INDENT_MARGIN,
                        );

                        let path_pb = gio_file.path().unwrap();
                        let settings = gio::Settings::new(APP_ID);
                        settings
                            .set_string("last-opened-dir", path_pb.to_str().unwrap())
                            .unwrap();
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
