use crate::core::dir::{render_children_dir, root_dir_subtitle, root_dir_title, INDENT_MARGIN};
use crate::APP_ID;
use adw::prelude::{ExpanderRowExt, PreferencesRowExt};
use adw::{gdk, gio, ExpanderRow, TabBar, TabView};
use gtk::gdk_pixbuf::Pixbuf;
use gtk::prelude::TextViewExt;
use gtk::prelude::*;
use gtk::Align::{Center, Start};
use gtk::Orientation::{Horizontal, Vertical};
use gtk::Overflow::Hidden;
use gtk::{PolicyType, TextBuffer, WrapMode};

const GAP: i32 = 4;
pub fn build_view(_win: &adw::ApplicationWindow) -> (gtk::Box, gtk::ScrolledWindow, TextBuffer) {
    let main_box = gtk::Box::new(Horizontal, 0);

    let root_expander = ExpanderRow::builder()
        .overflow(Hidden)
        .css_classes(vec!["root-expander"])
        // .icon_name("org.gnome.Software.Create")
        .title("No Project")
        .width_request(320)
        .expanded(false) // 默认不展开
        .build();

    let sidebar_scrolled = gtk::ScrolledWindow::builder()
        .vscrollbar_policy(PolicyType::Automatic)
        .hscrollbar_policy(PolicyType::Never)
        // .hscrollbar_policy(PolicyType::Automatic)
        .child(&root_expander)
        .width_request(320)
        .overflow(Hidden)
        .vexpand(true)
        .margin_bottom(45)
        .build();

    // 创建tab bar
    let tab_view = TabView::new();
    let tab_bar = TabBar::builder()
        .view(&tab_view)
        .autohide(true)
        // .end_action_widget()
        .expand_tabs(true)
        .build();

    // 创建文本显示区域
    let text_view = gtk::TextView::builder()
        .editable(false)
        .vexpand(true)
        .hexpand(true)
        .wrap_mode(WrapMode::Word)
        .build();
    let text_buffer = text_view.buffer();

    add_signal(&text_view);

    // 文本滚动窗口
    let tv_scroller = gtk::ScrolledWindow::builder()
        .vscrollbar_policy(PolicyType::Automatic)
        .hscrollbar_policy(PolicyType::Never)
        .child(&text_view)
        .build();

    // 创建右侧工具
    let tool_box = gtk::Box::builder()
        .orientation(Vertical)
        .margin_top(GAP)
        .margin_bottom(GAP)
        .spacing(GAP)
        .build();

    // 创建“查看源码”工具按钮
    // 加载和缩放 SVG 图标
    let pixbuf = Pixbuf::from_file_at_scale("asset/code_icon.svg", 16, 16, true)
        .expect("Failed to load SVG");
    let code_icon = gtk::Picture::for_pixbuf(&pixbuf);
    let source_btn = gtk::Button::builder()
        // .icon_name("text-x.gcode")
        .cursor(&gdk::Cursor::from_name("pointer", None).unwrap())
        .focus_on_click(true)
        .child(&code_icon)
        .tooltip_text("show source code of markdown")
        .has_frame(false)
        .halign(Center)
        .valign(Start)
        .margin_start(GAP)
        .margin_end(GAP)
        .opacity(0.8)
        .build();

    // 打开终端 工具按钮
    let terminal_btn = gtk::Button::builder()
        .icon_name("gnome-terminal-symbolic")
        .tooltip_text("open terminal")
        .has_frame(false)
        .halign(Center)
        .valign(Start)
        .margin_start(GAP)
        .margin_end(GAP)
        .build();

    // 分享按钮
    let share_btn = gtk::Button::builder()
        .icon_name("send-to-symbolic")
        .tooltip_text("share with your friends")
        .has_frame(false)
        .halign(Center)
        .valign(Start)
        .margin_start(GAP)
        .margin_end(GAP)
        .build();

    // 游戏按钮
    let game_btn = gtk::Button::builder()
        .icon_name("games-app-symbolic")
        .tooltip_text("share with your friends")
        .has_frame(false)
        .halign(Center)
        .valign(Start)
        .margin_start(GAP)
        .margin_end(GAP)
        .build();

    // 添加所有的工具按钮
    tool_box.append(&source_btn);
    tool_box.append(&terminal_btn);
    tool_box.append(&share_btn);
    tool_box.append(&game_btn);

    // 将这三个组件添加到主盒子
    main_box.append(&sidebar_scrolled);
    main_box.append(&tv_scroller);
    main_box.append(&tool_box);

    // initialize global settings
    let settings = gio::Settings::new(APP_ID);
    let open_method = settings.string("open-method-type");
    let last_dpath = settings.string("last-opened-dir");
    let dir = gio::File::for_path(last_dpath);

    // initialize dir sidebar
    if open_method == "reopen" {
        render_children_dir(&dir, &text_buffer, &root_expander, INDENT_MARGIN);
        root_expander.set_title(&root_dir_title(&dir));
        root_expander.set_subtitle(&root_dir_subtitle(&dir));
    }

    (main_box, sidebar_scrolled, text_buffer)
}

fn add_signal(textview: &gtk::TextView) {
    let _ = textview;
}
