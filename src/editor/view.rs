use adw;
use adw::glib::{closure, closure_local};
use gtk::prelude::*;
use gtk::prelude::{GtkWindowExt, TextViewExt, WidgetExt};
use gtk::Orientation::Horizontal;
use gtk::{PolicyType, ScrolledWindow, WrapMode};
use gtk::{Stack, StackSidebar};

pub fn build_view(_win: &adw::ApplicationWindow) -> gtk::Box {
    let stack = Stack::builder().hexpand(true).focus_on_click(true).build();

    let sidebar = StackSidebar::builder()
        .stack(&stack)
        .width_request(220)
        .build();

    let view_box = gtk::Box::builder()
        .vexpand(true)
        .orientation(Horizontal)
        .build();
    view_box.append(&sidebar);

    // 创建文本显示区域
    let text_view = gtk::TextView::builder()
        .editable(false)
        .vexpand(true)
        .hexpand(true)
        .wrap_mode(WrapMode::WordChar)
        .build();
    let text_buffer = text_view.buffer();

    text_view.connect_closure(
        "new-text",
        false,
        closure_local!(move |emitor, new_text| { text_buffer.set_text(new_text) }),
    );

    let tv_scroller = ScrolledWindow::builder()
        .vscrollbar_policy(PolicyType::Automatic)
        .hscrollbar_policy(PolicyType::Never)
        .child(&text_view)
        .build();

    // `stack` 监听 `dialog` 发射的 `folder-selected` 信号
    stack.connect_closure("directory", false, move |emitor, params| {
        stack.
    });

    view_box
}
