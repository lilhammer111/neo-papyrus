use adw::gio;
use adw::glib::BoxedAnyObject;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, ListView, SingleSelection, TreeListModel, TreeListRow, TreeExpander};
use gtk::{Box, Orientation};

fn main() {
    let app = gtk::Application::builder()
        .application_id("com.example.treeview")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &gtk::Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Simple TreeView")
        .default_width(600)
        .default_height(400)
        .build();

    // 创建根节点数据
    let root_items = vec!["Root 1", "Root 2"];

    // 使用 BoxedAnyObject 包装数据
    let root_model = gio::ListStore::new();
    for item in root_items {
        root_model.append(&BoxedAnyObject::new(item.to_string()));
    }

    // 创建 TreeListModel
    let tree_list_model = TreeListModel::new(
        Some(&root_model),
        false, // passthrough
        true,  // autoexpand
        Some(Box::new(move |item| {
            let item_str = item.item().unwrap().downcast_ref::<BoxedAnyObject>().unwrap().get::<String>().unwrap();
            let child_model = gio::ListStore::new();

            if item_str == "Root 1" {
                child_model.append(&BoxedAnyObject::new("Child 1.1".to_string()));
                child_model.append(&BoxedAnyObject::new("Child 1.2".to_string()));
            } else if item_str == "Root 2" {
                child_model.append(&BoxedAnyObject::new("Child 2.1".to_string()));
            }

            Some(child_model.upcast())
        }, 0)),
    );

    let selection_model = SingleSelection::new(Some(&tree_list_model));

    let factory = gtk::SignalListItemFactory::new();
    factory.connect_setup(move |_, list_item| {
        let expander = TreeExpander::new();
        let label = gtk::Label::new(None);
        expander.set_child(Some(&label));
        list_item.set_child(Some(&expander));
    });

    factory.connect_bind(move |_, list_item| {
        let label = list_item
            .child()
            .unwrap()
            .downcast::<TreeExpander>()
            .unwrap()
            .child()
            .unwrap()
            .downcast::<gtk::Label>()
            .unwrap();
        let item = list_item.item().unwrap().downcast_ref::<BoxedAnyObject>().unwrap().get::<String>().unwrap();
        label.set_text(&item);
    });

    let list_view = ListView::builder()
        .model(&selection_model)
        .factory(&factory)
        .build();

    let vbox = Box::new(Orientation::Vertical, 5);
    vbox.append(&list_view);

    window.set_child(Some(&vbox));
    window.present();
}
