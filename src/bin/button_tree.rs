use adw::prelude::*;
use adw::{gdk, gio};
use gtk::Align::Start;
use gtk::Orientation::{Horizontal, Vertical};
use gtk::{GestureClick, PolicyType, ScrolledWindow, WrapMode};
use std::str::from_utf8;

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
    let dpath = "/home/lilhammer/Documents/HammerMind/Programming/Rust";
    let dir = gio::File::for_path(dpath);

    if let Ok(file_iter) =
        dir.enumerate_children("*", gio::FileQueryInfoFlags::NONE, gio::Cancellable::NONE)
    {
        while let Some(file_info) = file_iter.next_file(gio::Cancellable::NONE).unwrap() {
            let icon_name = if file_info.file_type() == gio::FileType::Directory {
                // "filemanager-app-symbolic"
                "folder-symbolic"
            } else {
                // "yelp-page-task-symbolic"
                "text-x-generic-symbolic"
            };

            let fpathbuf = file_info.name();
            let file_name = fpathbuf.file_name().unwrap().to_str().unwrap();
            let file_path = dir.child(file_name);

            let btn = gtk::Button::builder()
                .child(
                    &adw::ButtonContent::builder()
                        .icon_name(icon_name)
                        .label(file_name)
                        .halign(Start)
                        .build(),
                )
                .build();

            // 为按钮添加点击事件
            let text_buffer = text_buffer.clone();
            // 创建 GestureClick 以处理双击事件
            let gesture = GestureClick::builder()
                .button(1)
                .propagation_phase(gtk::PropagationPhase::Capture)
                .build();

            gesture.connect_released(move |_, n_press, _, _| {
                if n_press == 2 {
                    if file_info.file_type() == gio::FileType::Regular {
                        if let Ok((contents, _)) = file_path.load_contents(gio::Cancellable::NONE) {
                            if let Ok(text) = from_utf8(&contents) {
                                text_buffer.set_text(text);
                            } else {
                                text_buffer.set_text("Failed to get file text");
                            }
                        }
                    }
                }
            });

            btn.add_controller(gesture);

            listbox.append(&btn);
        }
    }

    let scrolled_window = gtk::ScrolledWindow::builder()
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
