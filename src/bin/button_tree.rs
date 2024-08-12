use adw::prelude::*;
use adw::{gdk, gio};
use gtk::Align::Start;
use gtk::Orientation::{Horizontal, Vertical};
use gtk::{GestureClick, PolicyType, ScrolledWindow, WrapMode};
use std::cell::Cell;
use std::rc::Rc;
use std::str::from_utf8;
use uuid::Uuid;

fn main() {
    let app = adw::Application::builder()
        .application_id("com.example.expanderrow")
        .build();

    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);
    app.run();
}

fn load_css() {
    let css = "
    list {
        background-color: transparent; /* 将ListBox的背景色设置为白色 */
    }

    button {
        background-color: transparent; /* 将Button的背景色设置为透明 */
        padding: 0;  /* 将button的行高降到最小，因为gtk css没有Height属性 */
    }

    textview.view {
        padding: 10px 20px 10px 20px;
    }
    ";

    // 创建和加载 CSS Provider
    let provider = gtk::CssProvider::new();
    provider.load_from_data(css);

    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Error initializing GTK display"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    )
}

fn build_ui(app: &adw::Application) {
    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("ExpanderRow Example")
        .default_width(1000)
        .default_height(800)
        .build();

    // 创建文本显示区域
    let text_view = gtk::TextView::builder()
        .editable(false)
        .vexpand(true)
        .hexpand(true)
        .wrap_mode(WrapMode::WordChar)
        .build();
    let text_buffer = text_view.buffer();

    let tv_scroller = ScrolledWindow::builder()
        .vscrollbar_policy(PolicyType::Automatic)
        .hscrollbar_policy(PolicyType::Never)
        .child(&text_view)
        .build();

    let listbox = gtk::ListBox::builder().vexpand(true).build();

    // 从文件系统中读取文件
    let dpath = "/home/lilhammer/project/hammer-mind";
    let dir = gio::File::for_path(dpath);

    render_directory(&dir, &listbox, &text_buffer, None, 0, Some(Uuid::new_v4()));

    let scrolled_window = ScrolledWindow::builder()
        .width_request(240)
        .vexpand(true)
        .build();
    scrolled_window.set_child(Some(&listbox));

    let view_box = gtk::Box::new(Horizontal, 0);
    view_box.append(&scrolled_window);
    view_box.append(&tv_scroller);

    let mbox = gtk::Box::builder().orientation(Vertical).build();
    let header = adw::HeaderBar::new();
    mbox.append(&header);
    mbox.append(&view_box);

    window.set_content(Some(&mbox));
    window.show();
}

fn render_directory(
    dir: &gio::File,
    listbox: &gtk::ListBox,
    text_buffer: &gtk::TextBuffer,
    parent_row: Option<&gtk::ListBoxRow>, // 通过 Option 参数来插入子项
    depth: i32,
    parent_id: Option<Uuid>, // 父级ID，用于标记子项
) -> i32 {
    println!("passed pid: {}", parent_id.unwrap());

    let mut inserted_count = 0; // 记录插入的子项数量

    if let Ok(file_iter) =
        dir.enumerate_children("*", gio::FileQueryInfoFlags::NONE, gio::Cancellable::NONE)
    {
        let mut insert_position = if let Some(parent) = parent_row {
            let mut position = 0;
            while let Some(row) = listbox.row_at_index(position) {
                if row.eq(parent) {
                    break;
                }
                position += 1
            }
            position + 1
        } else {
            0 // 如果没有父级，插入到开头
        };

        while let Some(file_info) = file_iter.next_file(gio::Cancellable::NONE).unwrap() {
            let icon_name = if file_info.file_type() == gio::FileType::Directory {
                "folder-symbolic"
            } else {
                "text-x-generic-symbolic"
            };

            let pb = file_info.name();
            let file_name = pb.file_name().unwrap().to_str().unwrap();
            let file_path = dir.child(file_name);

            let btn = gtk::Button::builder()
                .child(
                    &adw::ButtonContent::builder()
                        .icon_name(icon_name)
                        .label(file_name)
                        .halign(Start)
                        .margin_start(depth * 20) // 根据深度设置左侧缩进
                        .build(),
                )
                .build();

            let listbox_clone = listbox.clone();
            let text_buffer_clone = text_buffer.clone();
            let file_path_clone = file_path.clone();

            let gesture = GestureClick::builder()
                .button(1)
                .propagation_phase(gtk::PropagationPhase::Capture)
                .build();

            let file_info_clone = file_info.clone();
            let btn_cloned = btn.clone();

            let row = gtk::ListBoxRow::new();
            let is_expanded = Rc::new(Cell::new(false));
            unsafe {
                row.set_data("is_expanded", is_expanded.clone());
                row.set_data("parent_id", parent_id);
            }
            let row_cloned = row.clone();
            let inserted_count_cloned = Rc::new(Cell::new(0)); // 新的计数器

            gesture.connect_released(move |_, n_press, _, _| {
                if n_press == 2 && file_info_clone.file_type() == gio::FileType::Regular {
                    // 如果是文件，则读取并显示内容
                    if let Ok((contents, _)) = file_path_clone.load_contents(gio::Cancellable::NONE)
                    {
                        if let Ok(text) = from_utf8(&contents) {
                            text_buffer_clone.set_text(text);
                        } else {
                            text_buffer_clone.set_text("Failed to get file text");
                        }
                    }
                } else if n_press == 1 && file_info_clone.file_type() == gio::FileType::Directory {
                    let is_expanded = unsafe {
                        row_cloned
                            .data::<Rc<Cell<bool>>>("is_expanded")
                            .unwrap()
                            .as_ref()
                    };
                    // 如果是目录，则插入子项

                    if is_expanded.get() {
                        // 如果已经展开，则收回子项
                        let mut remove_index = insert_position;
                        println!("insert_position: {insert_position}");
                        while let Some(child_row) = listbox_clone.row_at_index(remove_index+1) {
                            // 检查子项是否属于当前的父目录

                        }
                        is_expanded.set(false);
                    } else {
                        let clicked_row_id = Uuid::new_v4();
                        // 如果未展开，则插入子项
                        let added_count = render_directory(
                            &file_path_clone,
                            &listbox_clone,
                            &text_buffer_clone,
                            Some(
                                &btn_cloned
                                    .parent()
                                    .unwrap()
                                    .downcast::<gtk::ListBoxRow>()
                                    .unwrap(),
                            ),
                            depth + 1,
                            Some(clicked_row_id), // 传递当前ID
                        );
                        inserted_count_cloned.set(added_count);
                        is_expanded.set(true);
                    }
                }
            });

            btn.add_controller(gesture);

            row.set_child(Some(&btn));
            listbox.insert(&row, insert_position);
            insert_position += 1;
            inserted_count += 1;
        }
    }
    inserted_count
}
