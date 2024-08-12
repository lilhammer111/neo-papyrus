use adw::ExpanderRow;
use gtk::prelude::TextViewExt;
use gtk::prelude::*;
use gtk::Orientation::Horizontal;
use gtk::{PolicyType, ScrolledWindow, TextBuffer, WrapMode};
use gtk::Overflow::Hidden;

pub fn build_view(_win: &adw::ApplicationWindow) -> (gtk::Box, ExpanderRow, TextBuffer) {
    let main_box = gtk::Box::new(Horizontal, 0);

    let expander_row = ExpanderRow::builder()
        .css_classes(vec!["root-expander"])
        .title("No Project")
        .expanded(false) // 默认不展开
        .build();

    let sidebar_scrolled = ScrolledWindow::builder()
        .vscrollbar_policy(PolicyType::Automatic)
        .hscrollbar_policy(PolicyType::Never)
        // .hscrollbar_policy(PolicyType::Automatic)
        .child(&expander_row)
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

    (main_box, expander_row, text_buffer)
}
