use gtk::gio::{File, ListStore};
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, ListView, Orientation, TreeExpander, TreeListModel, Box, gio};

fn main() {
    let app = Application::builder()
        .application_id("com.example.fileexplorer")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &gtk::Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("File Explorer")
        .default_width(400)
        .default_height(300)
        .build();

    // 根目录
    let root_file = File::for_path("/path/to/directory");

    // 创建 ListStore
    let list_store: ListStore = ListStore::new();

    // 创建 TreeListModel
    let tree_list_model = TreeListModel::new();

    let list_view = ListView::builder()
        .model(&tree_list_model)
        .build();

    let vbox = Box::new(Orientation::Vertical, 5);
    vbox.append(&list_view);

    window.set_child(Some(&vbox));
    window.present();
}

fn create_directory_model(file: &File) -> Option<ListStore> {
    // 读取文件夹内容并填充到 ListStore 中
    let model = ListStore::new();
    if let Ok(children) = file.enumerate_children("standard::*", gio::FileQueryInfoFlags::NONE, None) {
        for child in children {
            if let Ok(child_info) = child {
                let child_file = file.resolve_relative_path(child_info.name());
                model.append(&child_file);
            }
        }
    }
    Some(model)
}
