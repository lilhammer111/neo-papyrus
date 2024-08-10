use gtk::prelude::*;
use gtk::Orientation::Horizontal;
use gtk::{Stack, StackSidebar};

pub fn build_view() -> gtk::Box {
    let stack = Stack::builder().hexpand(true).focus_on_click(true).build();

    let sidebar = StackSidebar::builder()
        .stack(&stack)
        .width_request(240)
        .build();

    let view_box = gtk::Box::builder()
        .vexpand(true)
        .orientation(Horizontal)
        .build();
    view_box.append(&sidebar);
    view_box
}
