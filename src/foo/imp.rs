use glib::subclass::object::ObjectImpl;
use glib::subclass::types::{ObjectSubclass, ObjectSubclassIsExt, InstanceStructExt};
use glib::subclass::InitializingObject;
use std::cell::RefCell;

#[derive(Default)]
pub struct Foo {
    pub a: RefCell<i32>,
    pub b: RefCell<i32>,
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("dropped Foo")
    }
}


#[glib::object_subclass]
impl ObjectSubclass for Foo {
    const NAME: &'static str = "Foo";
    const ABSTRACT: bool = false;
    type Type = super::Foo;
    type Class = super::ffi::FooClass;

    fn instance_init(_obj: &InitializingObject<Self>) {
        unsafe {
            let r = _obj.as_ref().imp();
            *(r.a.borrow_mut()) = 1;
            *(r.b.borrow_mut()) = 2;
        }
    }

    fn class_init(klass: &mut Self::Class) {
        klass.get_a = Some(foo_get_a);
        klass.set_a = Some(foo_set_a);
    }
}

pub trait FooImpl: ObjectImpl + 'static {}

impl ObjectImpl for Foo {}

impl Foo {
    pub fn a(&self) -> i32 {
        *(self.a.borrow())
    }
    pub fn set_a(&self, value: i32) {
        *(self.a.borrow_mut()) = value;
    }
    pub fn b(&self) -> i32 {
        *(self.b.borrow())
    }
    pub fn set_b(&self, value: i32) {
        *(self.b.borrow_mut()) = value;
    }
}

unsafe extern "C" fn foo_get_a(this: *mut super::ffi::Foo) -> i32 {
    (*this).imp().a()
}

unsafe extern "C" fn foo_set_a(this: *mut super::ffi::Foo, value: i32) {
    (*this).imp().set_a(value)
}

