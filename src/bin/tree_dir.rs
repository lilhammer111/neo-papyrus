use adw::glib::clone;
use gtk::gio::{Cancellable, File, FileQueryInfoFlags, FileType};
use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, CellRendererText, ScrolledWindow, TreeIter, TreeStore,
    TreeView, TreeViewColumn,
};

fn main() {
    let app = Application::builder()
        .application_id("com.example.fileexplorer")
        .build();

    app.connect_activate(build_ui);
    app.run();
}
fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("File Explorer")
        .default_width(600)
        .default_height(400)
        .build();

    let tree =


    let scrolled_window = ScrolledWindow::builder().child(&tree).build();
    window.set_child(Some(&scrolled_window));
    window.show();
}
