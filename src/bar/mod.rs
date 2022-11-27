mod imp;
mod ffi;

use glib::subclass::types::ObjectSubclassIsExt;
use crate::foo;

glib::wrapper! {
    pub struct Bar(ObjectSubclass<imp::Bar>) @extends foo::Foo;
}

impl Bar {
    pub fn c(&self) -> i32 { self.imp().c() }
    pub fn set_c(&self, value: i32) { self.imp().set_c(value) }
    pub fn d(&self) -> i32 { self.imp().d() }
    pub fn set_d(&self, value: i32) { self.imp().set_d(value) }
}
