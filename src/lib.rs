use adw::prelude::IsA;
use adw::{Application, ApplicationWindow};
use gtk::prelude::BoxExt;
use gtk::Orientation::{Horizontal, Vertical};
use gtk::Widget;

pub fn box_win<T: IsA<Widget>>(app: &Application, widget: &T) -> ApplicationWindow {
    let mbox = gtk::Box::builder()
        .orientation(Vertical)
        .spacing(10)
        .build();

    let header_bar = adw::HeaderBar::builder().build();
    mbox.append(&header_bar);

    let hbox = gtk::Box::builder().orientation(Horizontal).build();

    hbox.append(widget);
    mbox.append(&hbox);

    ApplicationWindow::builder()
        .application(app)
        .content(&mbox)
        .default_width(1000)
        .default_height(800)
        .title("test")
        .icon_name("document-open")
        .build()
}

pub fn frame_win<T: IsA<Widget>>(app: &Application, widget: &T) -> ApplicationWindow {
    let frame = gtk::Frame::builder().child(widget).build();

    ApplicationWindow::builder()
        .application(app)
        .content(&frame)
        .default_width(1000)
        .default_height(800)
        .title("test")
        .icon_name("document-open")
        .build()
}

pub fn layout_win<T: IsA<Widget>>(app: &Application, widget: &T) -> ApplicationWindow {
    let layout = gtk::BoxLayout::builder()
        .orientation(Vertical)
        .spacing(1)
        .build();

    let mbox = gtk::Box::builder().layout_manager(&layout).build();

    mbox.append(widget);


    ApplicationWindow::builder()
        .application(app)
        .content(&mbox)
        .default_width(1000)
        .default_height(800)
        .title("test")
        .icon_name("document-open")
        .build()
}

