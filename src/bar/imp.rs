use std::cell::RefCell;
use glib::subclass::InitializingObject;
use glib::subclass::object::ObjectImpl;
use glib::subclass::types::{ObjectSubclass, ObjectSubclassIsExt};
use crate::foo::FooImpl;

#[derive(Default)]
pub struct Bar {
    pub c: RefCell<i32>,
    pub d: RefCell<i32>,
}

impl Drop for Bar {
    fn drop(&mut self) {
        println!("dropped Bar")
    }
}

#[glib::object_subclass]
impl ObjectSubclass for Bar {
    const NAME: &'static str = "Bar";
    const ABSTRACT: bool = false;
    type ParentType = crate::foo::Foo;
    type Type = super::Bar;
    type Class = super::ffi::BarClass;

    fn instance_init(_obj: &InitializingObject<Self>) {
        unsafe {
            let r = _obj.as_ref().imp();
            *(r.c.borrow_mut()) = 3;
            *(r.d.borrow_mut()) = 4;
        }
    }
}

impl ObjectImpl for Bar {}

impl FooImpl for Bar {}

impl Bar {
    pub fn c(&self) -> i32 { *(self.c.borrow()) }
    pub fn set_c(&self, value: i32) { *(self.c.borrow_mut()) = value; }
    pub fn d(&self) -> i32 { *(self.d.borrow()) }
    pub fn set_d(&self, value: i32) { *(self.d.borrow_mut()) = value; }
}
