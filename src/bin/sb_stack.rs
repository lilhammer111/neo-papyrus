use gtk::glib::clone;
use gtk::prelude::*;
use gtk::Orientation::{Horizontal, Vertical};
use gtk::{Align, Application, ApplicationWindow, Box, Label, Stack, StackSidebar};

fn main() {
    let app = Application::new(Some("com.example.StackExample"), Default::default());
    app.connect_activate(|app| {
        build_ui(app);
    });
    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::new(app);
    window.set_title(Some("Stack Example"));
    window.set_default_size(800, 600);

    // 创建主盒子，包含左侧导航栏和右侧stack
    let main_box = Box::builder().orientation(Horizontal).spacing(0).build();

    // 创建 Stack
    let stack = Stack::builder().valign(Align::Start).build(); // valign start is key point
    stack.set_transition_type(gtk::StackTransitionType::SlideUpDown);
    stack.set_transition_duration(300);

    // 创建页面
    let page1 = Label::new(Some("This is Page 1"));
    let page2 = Label::new(Some("This is Page 2"));

    // 添加页面到 Stack
    stack.add_named(&page1, Some("page1"));
    stack.add_named(&page2, Some("page2"));

    let btn1 = gtk::Button::builder().icon_name("document-open").build();

    btn1.connect_clicked(clone!(
        #[strong]
        stack,
        move |_| stack.set_visible_child_name("page1")
    ));

    let btn2 = gtk::Button::builder().icon_name("document-edit").build();
    btn2.connect_clicked(clone!(
        #[strong]
        stack,
        move |_| stack.set_visible_child_name("page2")
    ));

    let vbox = Box::builder().orientation(Vertical).build();
    vbox.append(&btn1);
    vbox.append(&btn2);

    let sidebar = StackSidebar::builder().stack(&stack).build();

    // 将 sidebar 添加到猪盒子
    main_box.append(&vbox);
    // 将 Stack 添加到主盒子中
    main_box.append(&stack);

    window.set_child(Some(&main_box));
    window.present();
}
