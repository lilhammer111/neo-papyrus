use adw::prelude::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Label, Orientation, Image, IconSize};
use adw::{ ViewStack, ViewStackPage};

fn main() {
    let app = Application::new(Some("com.example.StackSidebarExample"), Default::default());
    app.connect_activate(|app| {
        build_ui(app);
    });
    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::new(app);
    window.set_title(Some("AdwStackSidebar Example"));
    window.set_default_size(800, 600);

    // Create StackView and StackSidebar
    let stack_view = ViewStack::new();
    let stack_sidebar = AdwStackSidebar::new();
    stack_sidebar.set_stack(&stack_view);

    // Create pages
    let page1 = AdwStackPage::new();
    let page2 = AdwStackPage::new();

    page1.set_title("Page 1");
    page2.set_title("Page 2");

    let icon1 = Image::from_icon_name(Some("document-open-symbolic"), IconSize::Button);
    let icon2 = Image::from_icon_name(Some("edit-find-symbolic"), IconSize::Button);

    let box1 = Box::builder().orientation(Orientation::Vertical).build();
    let box2 = Box::builder().orientation(Orientation::Vertical).build();

    box1.append(&icon1);
    box1.append(&Label::new(Some("Page 1 Content")));

    box2.append(&icon2);
    box2.append(&Label::new(Some("Page 2 Content")));

    page1.set_child(Some(&box1));
    page2.set_child(Some(&box2));

    stack_view.add_page(&page1);
    stack_view.add_page(&page2);

    // Add components to main window
    window.set_child(Some(&stack_sidebar));
    window.present();
}
