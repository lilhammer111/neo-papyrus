mod menu;
mod shortcuts;
mod view;

use crate::menu::build_menu;
use crate::shortcuts::setup_shortcuts;
use crate::view::build_view;
use adw::glib::ExitCode;
use adw::prelude::*;
use adw::{self, gdk};
use gtk::Align::{End, Start};
use gtk::Orientation::Vertical;
fn main() -> ExitCode {
    let app = adw::Application::builder()
        .application_id("hello.editor")
        .build();
    app.connect_activate(build_ui);
    app.connect_startup(setup_shortcuts);
    app.connect_startup(load_css);
    app.run()
}

fn load_css(_: &adw::Application) {
    let css = "
    button.file-btn {
        min-height: 30px;
        padding: 0;
        background-color: transparent; /* 将Button的背景色设置为透明 */
    }

    .root-expander {
        border-bottom-style: none;
    }

    row.root-expander row {
        padding-left: 0;
        padding-right: 0;
    }

    row.dir-expander > box {
        padding-left: 20px;
    }

    row.dir-expander > box > list > row > box {
        min-height: 30px;
        padding-left: 0;
        padding-right: 0;
        margin-left: 0;
        margin-right: 0;
    }

    row.dir-expander > box > list > row, row.dir-expander {
        padding-left: 0;
        padding-right: 0;
        padding-top: 0;
        padding-bottom: 0;
    }

    textview.view {
        padding: 10px 20px 10px 20px;
    }

    list {
        background-color: transparent; /* 将ListBox的背景色设置为白色 */
    }

    image {
        margin-right: 6px;
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
    let win = adw::ApplicationWindow::builder()
        .title("Editor")
        .default_width(1000)
        .default_height(780)
        .application(app)
        .build();

    let header_bar = adw::HeaderBar::builder().build();
    let menu = build_menu(&win);

    let mbox = gtk::Box::builder().orientation(Vertical).build();

    let view = build_view(&win);
    mbox.append(&header_bar);
    mbox.append(&menu);
    mbox.append(&view);

    let overlay = gtk::Overlay::builder().child(&mbox).build();

    let btn = gtk::Button::builder()
        .has_frame(false)
        .icon_name("list-add-symbolic")
        .halign(Start)
        .valign(End)
        .margin_bottom(10)
        .margin_start(10)
        .build();
    overlay.add_overlay(&btn);

    win.set_content(Some(&overlay));
    win.show();
}
