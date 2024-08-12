use adw::prelude::{ExpanderRowExt, FileEnumeratorExt, FileExt, FileExtManual};
use adw::{gio, ExpanderRow};
use gtk::prelude::{TextBufferExt, WidgetExt};
use gtk::Align::Start;
use gtk::GestureClick;
use std::str::from_utf8;

pub fn render_directory(
    dir: &gio::File,
    text_buffer: &gtk::TextBuffer,
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
                render_directory(&file_path, &text_buffer, &child_expander, depth);
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
                let text_buffer_clone = text_buffer.clone();
                gesture.connect_released(move |_, n_press, _, _| {
                    if n_press == 2 {
                        if let Ok((contents, _)) =
                            file_path_clone.load_contents(gio::Cancellable::NONE)
                        {
                            if let Ok(text) = from_utf8(&contents) {
                                text_buffer_clone.set_text(text);
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
