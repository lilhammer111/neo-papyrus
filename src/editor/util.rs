use adw::prelude::{ExpanderRowExt, FileEnumeratorExt, FileExt, FileExtManual};
use adw::{gio, ExpanderRow};
use gtk::prelude::{TextBufferExt, WidgetExt};
use gtk::Align::Start;
use gtk::GestureClick;
use std::process::Command;
use std::str::from_utf8;

pub const INDENT_MARGIN: i32 = 20;
pub fn render_children_dir(
    dir: &gio::File,
    text_buffer: &gtk::TextBuffer,
    parent_expander: &ExpanderRow, // 通过 Option 参数来插入子项
    indent_margin: i32,
) {
    if let Ok(file_iter) =
        dir.enumerate_children("*", gio::FileQueryInfoFlags::NONE, gio::Cancellable::NONE)
    {
        // 初始化子项用于设置expander 的 enable_expansion
        let mut child_count = 0;

        // 遍历传进来的dir，将文件转为btn， 将目录转为expander
        while let Some(file_info) = file_iter.next_file(gio::Cancellable::NONE).unwrap() {
            // 获取文件类型 file_kind，文件名 file_name， 文件路径 fpath
            let pb = file_info.name();
            let file_name = pb.to_str().unwrap();
            let file_kind = file_info.file_type();
            let content_type = file_info.content_type().unwrap();
            // println!("file name: {}, content type: {}", file_name, content_type);

            if should_skip(&file_name, file_kind, &content_type) {
                continue;
            }

            let file_path = dir.child(file_name);
            if file_kind == gio::FileType::Directory {
                // 如果文件类型为目录

                // 设置相应的item icon
                // let icon_name = "inode-directory-symbolic";
                let icon_name = "system-file-manager";


                // 为该子文件夹创建一个expander
                let child_expander = ExpanderRow::builder()
                    .css_classes(vec!["dir-expander"])
                    .icon_name(icon_name)
                    .title(file_name)
                    .expanded(false) // 默认不展开
                    .build();

                // 将子文件夹expander添加到父expander中
                parent_expander.add_row(&child_expander);
                child_count += 1;

                // 递归调用 render_directory方法来创建子文件夹的子项
                render_children_dir(&file_path, &text_buffer, &child_expander, indent_margin);
            } else {
                // 否则文件类型为常规文件 Regular

                // 设置文件icon
                let icon_name = content_type_to_icon(content_type.as_str());

                // 设置相应的item icon

                // 为该子文件创建按钮
                let btn = gtk::Button::builder()
                    .margin_start(indent_margin) // 根据深度设置左侧缩进
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
                child_count += 1;

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

        if child_count == 0 {
            parent_expander.set_enable_expansion(false);
            parent_expander.set_opacity(0.4);
        }
    }
}

#[cfg(target_os = "linux")]
#[allow(unused)]
fn mime_type(fpath: &str) -> std::io::Result<()> {
    let output = Command::new("file")
        .arg("--mime-type")
        .arg(fpath)
        .output()?;
    let output_str = String::from_utf8_lossy(&output.stdout);
    println!("{}", output_str);
    Ok(())
    // Ok(output_str.trim().to_string())
}

/// Determines whether a file should be skipped based on its name and content type.
/// Skips hidden files (those starting with a dot) and files that are neither markdown
/// nor plain text. Directories are not skipped.
///
/// # Arguments
/// * `file_name` - The name of the file to check.
/// * `file_kind` - The type of the file, expecting values from `gio::FileType`.
/// * `content_type` - The MIME type of the file as a string slice.
///
/// # Return
/// Returns `true` if the file should be skipped, otherwise `false`.
fn should_skip(file_name: &str, file_kind: gio::FileType, content_type: &str) -> bool {
    let is_hidden_file = file_name.starts_with(".");
    let is_not_plain_text = content_type != "text/plain";
    let is_not_markdown = content_type != "text/markdown";

    is_hidden_file
        || (file_kind == gio::FileType::Regular && (is_not_plain_text && is_not_markdown))
}

fn content_type_to_icon(content_type: &str) -> String {
    match content_type {
        "text/markdown" => String::from("text-markdown"),
        _ => String::from("document"),
    }
}
