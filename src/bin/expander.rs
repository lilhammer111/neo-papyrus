use adw::prelude::*;
use adw::{gdk, ExpanderRow};
use gtk::Align::{Center, Start};

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
    box.header {
        min-height: 30px;
        margin: 0;
    }

    button {
        padding: 0;
        min-height: 30px;
    }

    image {
        margin-right: 6px;
    }
";

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
        .valign(Center)
        .expanded(false) // 默认不展开
        .build();

    // 在展开区域中添加一个示例标签
    let btn1 = gtk::Button::builder()
        .child(
            &adw::ButtonContent::builder()
                .icon_name("folder-symbolic")
                .label("RUst")
                .halign(Start)
                .margin_start(20) // 根据深度设置左侧缩进
                .build(),
        )
        .has_frame(false)
        .build();

    let expander_row2 = ExpanderRow::builder()
        .icon_name("filemanager-app-symbolic")
        .title("Advanced Options")
        .valign(Center)
        .expanded(false) // 默认不展开
        .margin_start(20)
        .build();

    let btn2 = gtk::Button::builder()
        .child(
            &adw::ButtonContent::builder()
                .icon_name("folder-symbolic")
                .label("RUst")
                .halign(Start)
                .margin_start(40) // 根据深度设置左侧缩进
                .build(),
        )
        .build();
    expander_row2.add_row(&btn2);

    expander_row.add_row(&btn1);
    expander_row.add_row(&expander_row2);

    let vbox = gtk::ListBox::builder().build();

    // 将 ExpanderRow 添加到 ListBox 中
    vbox.append(&expander_row);

    window.set_content(Some(&vbox));
    window.show();
}
