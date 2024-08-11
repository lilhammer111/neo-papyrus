// use adw::{gio, glib};
use gtk::prelude::*;
use gtk::Orientation::Horizontal;
use gtk::{ApplicationWindow, Stack, StackSidebar};

pub  fn build_view(win: &ApplicationWindow) -> gtk::Box {
    let stack = Stack::builder().hexpand(true).focus_on_click(true).build();

    let sidebar = StackSidebar::builder()
        .stack(&stack)
        .width_request(220)
        .build();

    let view_box = gtk::Box::builder()
        .vexpand(true)
        .orientation(Horizontal)
        .build();
    view_box.append(&sidebar);

    
    // `stack` 监听 `dialog` 发射的 `folder-selected` 信号
    // stack.connect_closure("folder-selected", false, move |values: &[glib::Value]| {
    //     let folder = values[0].get::<gio::File>().expect("Expected gio::File");
    //     let file_enumerator = folder
    //         .enumerate_children(
    //             "*",
    //             gio::FileQueryInfoFlags::NONE,
    //             gio::Cancellable::NONE,
    //         )
    //         .expect("failed to enumerate folder's children");
    //
    //     while let Some(info) = file_enumerator
    //         .next_file(gio::Cancellable::NONE)
    //         .expect("Failed to get next file")
    //     {
    //         let file_name = info.name().expect("Failed to get file name");
    //         let label = gtk::Label::new(Some(&format!("File: {}", file_name)));
    //         stack.add_titled(&label, &file_name, &file_name);
    //     }
    // });

    view_box
}
