use adw::{Application, ApplicationWindow, TabBar, TabPage, TabView};
use adw::prelude::AdwApplicationWindowExt;
use gtk::prelude::*;
use gtk::{Box, Orientation, TextView};

fn main() {
    let app = Application::builder()
        .application_id("com.example.TabBarExample")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    // 创建主窗口
    let window = ApplicationWindow::builder()
        .application(app)
        .title("TabBar Example")
        .default_width(600)
        .default_height(400)
        .build();

    // 创建水平布局
    let hbox = Box::new(Orientation::Vertical, 0);

    // 创建TabBar和TabView
    let tab_bar = TabBar::new();
    let tab_view = TabView::new();

    tab_bar.set_view(Option::from(&tab_view));

    // 添加TabBar到布局
    hbox.append(&tab_bar);

    // 添加多个TextView到TabView作为标签页的内容
    for i in 1..=3 {
        let text_view = TextView::new();
        text_view.buffer().set_text(&format!("This is tab {}", i));
        let title = format!("Tab {}", i);

        let page = tab_view.append(&text_view);
        page.set_title(&title)
    }

    // 添加TabView到布局
    hbox.append(&tab_view);

    // 将布局添加到主窗口
    window.set_content(Some(&hbox));

    // 显示所有窗口内容
    window.show();
}
