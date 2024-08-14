use crate::core::dir::{render_children_dir, INDENT_MARGIN, root_dir_title, root_dir_subtitle};
use crate::APP_ID;
use adw::gio::{ActionEntry, SimpleActionGroup};
use adw::prelude::{ActionMapExtManual, FileExt, SettingsExt};
use adw::{gio, ExpanderRow};
use gtk::prelude::{DialogExt, FileChooserExt, GtkWindowExt, WidgetExt};
use gtk::{FileChooserAction, Overflow, ResponseType};

pub fn file_actions(
    win: &adw::ApplicationWindow,
    scrl_window: &gtk::ScrolledWindow,
    tabview: &adw::TabView,
) -> SimpleActionGroup {
    let action_new_proj = ActionEntry::builder("newp")
        .activate(move |_, _, _| println!("new project"))
        .build();

    let win = win.clone();
    let scrl_window_cloned = scrl_window.clone();
    let tabview_cloned = tabview.clone();
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
            let scrl_window_cloned = scrl_window_cloned.clone();
            let tabview_cloned = tabview_cloned.clone();
            dialog.connect_response(move |dialog, resp_kind| {
                if resp_kind == ResponseType::Accept {
                    if let Some(g_dir) = dialog.file() {
                        // set project name to expander
                        // 通过设置一个新的root expander，来清空原先的root expander
                        let root_expander = ExpanderRow::builder()
                            // .icon_name("org.gnome.Software.Create")
                            .overflow(Overflow::Hidden)
                            .css_classes(vec!["root-expander"])
                            .width_request(320)
                            .subtitle(root_dir_subtitle(&g_dir))
                            .title(root_dir_title(&g_dir))
                            .expanded(false) // 默认不展开
                            .build();
                        scrl_window_cloned.set_child(Some(&root_expander));

                        // 递归生成子目录
                        render_children_dir(
                            &g_dir,
                            &tabview_cloned,
                            &root_expander,
                            INDENT_MARGIN,
                        );

                        // 更新上次打开的项目
                        let path_pb = g_dir.path().unwrap();
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
