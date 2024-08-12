use adw::gio::{ActionEntry, Menu, MenuModel, SimpleActionGroup};
use adw::prelude::{
    ActionMapExtManual, Cast, ExpanderRowExt, FileEnumeratorExt, FileExt, FileExtManual, ObjectExt,
};
use adw::ExpanderRow;
use gtk::prelude::{
    DialogExt, FileChooserExt, GtkWindowExt, TextBufferExt, TextViewExt, WidgetExt,
};
use gtk::Align::Start;
use gtk::{
    gio, FileChooserAction, GestureClick, PolicyType, PopoverMenuBar, ResponseType, ScrolledWindow,
    WrapMode,
};
use std::str::from_utf8;

pub fn build_menu(win: &adw::ApplicationWindow) -> PopoverMenuBar {
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

fn create_actions(win: &adw::ApplicationWindow) -> SimpleActionGroup {
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
                    if let Some(gio_file) = dialog.file() {
                        let expander_row = ExpanderRow::builder()
                            .css_classes(vec!["root-expander"])
                            .icon_name("filemanager-app-symbolic")
                            .title("Advanced Options")
                            .expanded(false) // 默认不展开
                            .build();

                        // render_directory(&gio_file, &text_buffer, &expander_row, 20);
                        render_directory(&gio_file, &expander_row, 20);

                        let scrolled_window = ScrolledWindow::builder()
                            .vscrollbar_policy(PolicyType::Automatic)
                            .hscrollbar_policy(PolicyType::Never)
                            .child(&expander_row)
                            .build();

                        dialog.emit_by_name("directory", &[&scrolled_window])
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

fn render_directory(
    dir: &gio::File,
    // text_buffer: &gtk::TextBuffer,
    parent_expander: &ExpanderRow, // 通过 Option 参数来插入子项
    depth: i32,
) {
    if let Ok(file_iter) =
        dir.enumerate_children("*", gio::FileQueryInfoFlags::NONE, gio::Cancellable::NONE)
    {
        // 遍历传进来的dir，将文件转为btn， 将目录转为expander
        while let Some(file_info) = file_iter.next_file(gio::Cancellable::NONE).unwrap() {
            // 获取文件类型 file_kind，文件名 fname， 文件路径 fpath
            let file_kind = file_info.file_type();
            let pb = file_info.name();
            let file_name = pb.file_name().unwrap().to_str().unwrap();
            let file_path = dir.child(file_name);

            if file_name == ".git" {
                continue;
            }

            if file_kind == gio::FileType::Directory {
                // 如果文件类型为目录

                // 设置相应的item icon
                let icon_name = "folder-symbolic";

                // 为该子文件夹创建一个expander
                let child_expander = ExpanderRow::builder()
                    .css_classes(vec!["dir-expander"])
                    .icon_name(icon_name)
                    .title(file_name)
                    .expanded(false) // 默认不展开
                    .build();

                // 将子文件夹expander添加到父expander中
                parent_expander.add_row(&child_expander);

                // 递归调用 render_directory方法来创建子文件夹的子项
                // render_directory(&file_path, &text_buffer, &child_expander, depth);
                render_directory(&file_path, &child_expander, depth);
            } else {
                // 否则文件类型为常规文件 Regular

                // 设置相应的item icon
                let icon_name = "text-x-generic-symbolic";

                // 为该子文件创建按钮
                let btn = gtk::Button::builder()
                    .margin_start(depth) // 根据深度设置左侧缩进
                    .child(
                        &adw::ButtonContent::builder()
                            .icon_name(icon_name)
                            .height_request(30)
                            .label(file_name)
                            .halign(Start)
                            .build(),
                    )
                    .css_classes(vec!["file-btn"])
                    .has_frame(false)
                    .build();

                // 将子文件添加到传入的父节点expander
                parent_expander.add_row(&btn);

                // 为子文件添加双击事件： 渲染文本内容到右侧text view
                let gesture = GestureClick::builder()
                    .button(1)
                    .propagation_phase(gtk::PropagationPhase::Capture)
                    .build();

                let file_path_clone = file_path.clone();
                // let text_buffer_clone = text_buffer.clone();
                let btn_cloned = btn.clone();
                gesture.connect_released(move |_, n_press, _, _| {
                    if n_press == 2 {
                        if let Ok((contents, _)) =
                            file_path_clone.load_contents(gio::Cancellable::NONE)
                        {
                            if let Ok(text) = from_utf8(&contents) {
                                // text_buffer_clone.set_text(text);
                                // todo send signals here
                                btn_cloned.emit_by_name("new-text", &[&text]);
                            } else {
                                // text_buffer_clone.set_text("Failed to get file text");
                                println!("failed to get file text")
                            }
                        }
                    }
                });

                btn.add_controller(gesture);
            }
        }
    }
}
