use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Label, Orientation, Stack, StackSwitcher};

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

    // 创建主盒子，包含顶部导航栏和下方内容
    let main_box = Box::new(Orientation::Vertical, 0);

    // 创建 Stack
    let stack = Stack::new();
    stack.set_transition_type(gtk::StackTransitionType::SlideLeftRight);
    stack.set_transition_duration(300);

    // 创建 StackSwitcher
    let stack_switcher = StackSwitcher::new();
    stack_switcher.set_stack(Some(&stack));

    // 创建页面
    let page1 = Label::new(Some("This is Page 1"));
    let page2 = Label::new(Some("This is Page 2"));

    // 添加页面到 Stack
    stack.add_titled(&page1, Some("page1"), "Page 1");
    stack.add_titled(&page2, Some("page2"), "Page 2");

    // 将 StackSwitcher 添加到顶部导航栏
    main_box.append(&stack_switcher);
    // 将 Stack 添加到主盒子中
    main_box.append(&stack);

    window.set_child(Some(&main_box));
    window.present();
}
