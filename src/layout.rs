use adw::{Application};
use gtk::prelude::BoxExt;
use gtk::Orientation::Horizontal;
use gtk::{Align, ApplicationWindow, Stack, StackSidebar};

pub struct View {
    pub stack: Stack,
    pub window: ApplicationWindow,
}

impl View {
    pub fn new(stack: Stack, window: ApplicationWindow) -> Self {
        Self { stack, window }
    }

    pub fn basic(app: &Application) -> Self {
        let stack = Stack::builder()
            .margin_start(10)
            .margin_end(10)
            .valign(Align::Start) // valign start is key point
            .transition_type(gtk::StackTransitionType::SlideUpDown)
            .transition_duration(300)
            .build();

        let sidebar = StackSidebar::builder().stack(&stack).build();

        let layout = gtk::Box::builder().orientation(Horizontal).build();

        layout.append(&sidebar);
        layout.append(&stack);

        let window = ApplicationWindow::builder()
            .application(app)
            .maximized(true)
            .height_request(600)
            .width_request(800)
            .child(&layout)
            .build();

        Self { stack, window }
    }
}
