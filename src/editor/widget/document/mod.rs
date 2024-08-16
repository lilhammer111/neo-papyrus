mod imp;
use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct TextObject(ObjectSubclass<imp::TextObject>);
}

impl TextObject {
    pub fn new(number: i32) -> Self {
        Object::builder().property("number", number).build()
    }
}
