use crate::util::{render_children_dir, INDENT_MARGIN};
use crate::APP_ID;
use adw::prelude::PreferencesRowExt;
use adw::{gio, ExpanderRow};
use gtk::prelude::TextViewExt;
use gtk::prelude::*;
use gtk::Orientation::Horizontal;
use gtk::Overflow::Hidden;
use gtk::{PolicyType, ScrolledWindow, TextBuffer, WrapMode};

pub fn build_view(_win: &adw::ApplicationWindow) -> (gtk::Box, ScrolledWindow, TextBuffer) {
    let main_box = gtk::Box::new(Horizontal, 0);

    let root_expander = ExpanderRow::builder()
        .overflow(Hidden)
        .css_classes(vec!["root-expander"])
        .title("No Project")
        .expanded(false) // 默认不展开
        .build();

    let sidebar_scrolled = ScrolledWindow::builder()
        .vscrollbar_policy(PolicyType::Automatic)
        .hscrollbar_policy(PolicyType::Never)
        // .hscrollbar_policy(PolicyType::Automatic)
        .child(&root_expander)
        .width_request(280)
        .overflow(Hidden)
        .max_content_width(400)
        .vexpand(true)
        .margin_bottom(45)
        .build();

    main_box.append(&sidebar_scrolled);

    // 创建文本显示区域
    let text_view = gtk::TextView::builder()
        .editable(true)
        .vexpand(true)
        .hexpand(true)
        .wrap_mode(WrapMode::Word)
        .build();
    let text_buffer = text_view.buffer();

    let tv_scroller = ScrolledWindow::builder()
        .vscrollbar_policy(PolicyType::Automatic)
        .hscrollbar_policy(PolicyType::Never)
        .child(&text_view)
        .build();

    main_box.append(&tv_scroller);

    // initialize global settings
    let settings = gio::Settings::new(APP_ID);
    let open_method = settings.string("open-method-type");
    let last_dpath = settings.string("last-opened-dir");
    let dir = gio::File::for_path(last_dpath);

    // initialize dir sidebar
    if open_method == "reopen" {
        render_children_dir(&dir, &text_buffer, &root_expander, INDENT_MARGIN);
        let pb = dir.basename().unwrap();
        root_expander.set_title(pb.to_str().unwrap());
    }

    (main_box, sidebar_scrolled, text_buffer)
}
