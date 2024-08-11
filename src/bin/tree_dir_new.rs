use adw::glib::clone;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, CellRendererText, TreeView, TreeViewColumn, TreeStore, TreeIter, ScrolledWindow};
use gtk::gio::{File, FileType, FileQueryInfoFlags, Cancellable};

fn main() {
    let app = Application::builder()
        .application_id("com.example.fileexplorer")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn populate_tree(store: &TreeStore, parent: Option<&TreeIter>, path: &str) {
    let folder = File::for_path(path);
    if let Ok(children) = folder.enumerate_children("standard::name,standard::type", FileQueryInfoFlags::NONE, Cancellable::NONE) {
        for child in children.filter_map(Result::ok) {
            let name = child.name();
            let file_type = child.file_type();
            let iter = store.append(parent);

            store.set(&iter, &[(0, &name)]);

            if file_type == FileType::Directory {
                // Insert a dummy child node to make the row expandable
                store.append(Some(&iter));
            }
        }
    }
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("File Explorer")
        .default_width(600)
        .default_height(400)
        .build();

    let store = TreeStore::new(&[String::static_type()]);
    let tree = TreeView::with_model(&store);
    let column = TreeViewColumn::new();
    let cell = CellRendererText::new();

    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", 0);
    tree.append_column(&column);

    tree.connect_row_expanded(clone!(#[strong] store ,
        move |tree_view, iter, _path| {
        // Check if the child is a dummy
        if store.iter_n_children(Some(iter)) == 1 && store.value(&store.iter_children(Some(iter)).unwrap(), 0).get::<String>().unwrap().is_empty() {
            store.remove(&store.iter_children(Some(iter)).unwrap());
            let path = format!("{}/{}", "/home/lilhammer/Documents/HammerMind", store.value(iter, 0).get::<String>().unwrap());
            populate_tree(&store, Some(iter), &path);
        }
    }));

    // Initial population at the root path
    populate_tree(&store, None, "/home/lilhammer/Documents/HammerMind");

    let scrolled_window = ScrolledWindow::builder().child(&tree).build();
    window.set_child(Some(&scrolled_window));
    window.show();
}
