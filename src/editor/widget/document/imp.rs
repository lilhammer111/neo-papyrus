use std::cell::Cell;

use glib::Properties;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

// Object holding the state
#[derive(Properties, Default)]
#[properties(wrapper_type = super::IntegerObject)]
pub struct TextObject {
    #[property(get, set)]
    text: Cell<String>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for TextObject {
    const NAME: &'static str = "DocumentTextSegment";
    type Type = super::IntegerObject;
}

// Trait shared by all GObjects
#[glib::derived_properties]
impl ObjectImpl for TextObject {}
