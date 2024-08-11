use adw::prelude::*;
use adw::{gdk, ExpanderRow};

fn main() {
    let app = adw::Application::builder()
        .application_id("com.example.expanderrow")
        .build();

    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    app.run();
}

fn load_css() {
    let css = "row {
    padding: 0;
    margin: 0;
}";

    // 创建和加载 CSS Provider
    let provider = gtk::CssProvider::new();
    provider.load_from_data(css);

    // 将 CSS Provider 应用到应用程序的默认屏幕
    gtk::StyleContext::add_provider_for_display(
        &gdk::Display::default().expect("Error initializing GTK display"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &adw::Application) {
    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("ExpanderRow Example")
        .default_width(400)
        .default_height(300)
        .build();

    let expander_row = ExpanderRow::builder()
        .icon_name("filemanager-app-symbolic")
        .title("Advanced Options")
        .expanded(false) // 默认不展开
        .build();

    // 在展开区域中添加一个示例标签

    let label = gtk::Label::new(Some("Here are some advanced options"));
    expander_row.add_row(&label);

    // 在展开区域中添加一个示例标签
    let label2 = gtk::Label::new(Some("Here are some advanced options"));
    expander_row.add_row(&label2);
    // 在展开区域中添加一个示例标签
    let label3 = gtk::Label::new(Some("Here are some advanced options"));
    expander_row.add_row(&label3);

    let expander_row2 = ExpanderRow::builder()
        .icon_name("filemanager-app-symbolic")
        .title("Advanced Options")
        .expanded(false) // 默认不展开
        .build();

    let label4 = gtk::Label::new(Some("Here are some advanced options"));
    expander_row2.add_row(&label4);
    // 在展开区域中添加一个示例标签
    let label5 = gtk::Label::new(Some("Here are some advanced options"));
    expander_row2.add_row(&label5);

    expander_row.add_row(&expander_row2);

    let vbox = gtk::ListBox::builder().build();

    // 将 ExpanderRow 添加到 ListBox 中
    vbox.append(&expander_row);

    window.set_content(Some(&vbox));
    window.show();
}
