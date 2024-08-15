use crate::core::dir::{render_children_dir, root_dir_subtitle, root_dir_title, INDENT_MARGIN};
use crate::core::parser::markdown::widget_from;
use crate::APP_ID;
use adw::prelude::{ExpanderRowExt, PreferencesRowExt};
use adw::{gdk, gio, ExpanderRow, TabBar, TabView};
use gtk::gdk_pixbuf::Pixbuf;
use gtk::prelude::*;
use gtk::Align::{Center, Start};
use gtk::Orientation::{Horizontal, Vertical};
use gtk::Overflow::Hidden;
use gtk::PolicyType;
use std::str::from_utf8;

const GAP: i32 = 4;
pub fn build_view(_win: &adw::ApplicationWindow) -> (gtk::Box, gtk::ScrolledWindow, TabView) {
    let main_box = gtk::Box::new(Horizontal, 0);

    let (sidebar, root_expander) = build_sidebar_ui(&main_box);

    let tabview = build_text_ui(&main_box);
    build_tool_ui(&main_box, &tabview);

    // initialize global settings
    let settings = gio::Settings::new(APP_ID);
    let open_method = settings.string("open-method-type");
    let last_dpath = settings.string("last-opened-dir");
    let dir = gio::File::for_path(last_dpath);

    // initialize dir sidebar
    if open_method == "reopen" {
        render_children_dir(&dir, &tabview, &root_expander, INDENT_MARGIN);
        root_expander.set_title(&root_dir_title(&dir));
        root_expander.set_subtitle(&root_dir_subtitle(&dir));
    }

    (main_box, sidebar, tabview)
}

#[allow(unused)]
fn add_signal(textview: &gtk::TextView) {
    let _ = textview;
}

fn build_sidebar_ui(main_box: &gtk::Box) -> (gtk::ScrolledWindow, ExpanderRow) {
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
    main_box.append(&sidebar_scrolled);

    (sidebar_scrolled, root_expander)
}

fn build_text_ui(main_box: &gtk::Box) -> TabView {
    // 创建tab bar
    let tab_view = TabView::new();
    let tab_bar = TabBar::builder()
        .name("tab-bar")
        .view(&tab_view)
        .autohide(true)
        // .end_action_widget()
        .expand_tabs(false)
        .build();

    // 文本滚动窗口
    let tv_scroller = gtk::ScrolledWindow::builder()
        .margin_top(30)
        .margin_bottom(30)
        .vscrollbar_policy(PolicyType::Automatic)
        .hscrollbar_policy(PolicyType::Never)
        .vexpand(true)
        .child(&tab_view)
        .build();

    let vbox = gtk::Box::new(Vertical, 0);
    vbox.append(&tab_bar);
    vbox.append(&tv_scroller);

    main_box.append(&vbox);

    tab_view
}

fn build_tool_ui(main_box: &gtk::Box, tabview: &TabView) {
    // 创建右侧工具
    let tool_box = gtk::Box::builder()
        .orientation(Vertical)
        .margin_top(GAP)
        .margin_bottom(GAP)
        .spacing(GAP)
        .build();

    // 创建“查看源码”工具按钮
    // 加载和缩放 SVG 图标
    let source_btn = gtk::ToggleButton::builder()
        // .icon_name("text-x.gcode")
        .cursor(&gdk::Cursor::from_name("pointer", None).unwrap())
        .focus_on_click(true)
        .child(&gtk::Picture::for_pixbuf(
            &Pixbuf::from_file_at_scale("asset/source-code.svg", 16, 16, true)
                .expect("Failed to load SVG"),
        ))
        .tooltip_text("show source code of markdown")
        .has_frame(false)
        .halign(Center)
        .valign(Start)
        .margin_start(GAP)
        .margin_end(GAP)
        .opacity(0.9)
        .build();

    let tabview_cloned = tabview.clone();
    source_btn.connect_toggled(move |btn| {
        if let Some(page) = tabview_cloned.selected_page() {
            println!("{}", page.title());
            unsafe {
                let fpath = page.data::<String>("fpath").unwrap();
                let fpath = fpath.as_ref();
                println!("fpath got from data: {}", fpath);
                if let Some(textview) = page.child().downcast_ref::<gtk::TextView>() {
                    let buf = textview.buffer();

                    let gfile = gio::File::for_path(fpath);
                    if let Ok((contents, _)) = gfile.load_contents(gio::Cancellable::NONE) {
                        let text = from_utf8(&contents).unwrap();
                        if btn.is_active() {
                            buf.set_text(text);
                        } else {
                            buf.set_text("");
                            let document_box = widget_from(&text);
                            let new_page = tabview_cloned.append(&document_box);
                            tabview_cloned.set_selected_page(&new_page);
                        }
                    }
                }
            }
        }
    });

    // 打开终端 工具按钮
    let terminal_btn = gtk::Button::builder()
        .cursor(&gdk::Cursor::from_name("pointer", None).unwrap())
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
        .cursor(&gdk::Cursor::from_name("pointer", None).unwrap())
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
        .cursor(&gdk::Cursor::from_name("pointer", None).unwrap())
        .icon_name("games-app-symbolic")
        .tooltip_text("share with your friends")
        .has_frame(false)
        .halign(Center)
        .valign(Start)
        .margin_start(GAP)
        .margin_end(GAP)
        .build();

    // shortcut 按钮
    let shortcut_icon = gtk::Picture::for_pixbuf(
        &Pixbuf::from_file_at_scale("asset/command.svg", 16, 16, true).unwrap(),
    );
    let shortcut_btn = gtk::Button::builder()
        .cursor(&gdk::Cursor::from_name("pointer", None).unwrap())
        .focus_on_click(true)
        .child(&shortcut_icon)
        .tooltip_text("show shortcut for the app")
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
    tool_box.append(&shortcut_btn);

    // 将这三个组件添加到主盒子
    main_box.append(&tool_box);
}
