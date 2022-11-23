use glib::subclass::object::ObjectImpl;
use glib::subclass::types::{ClassStruct, ObjectSubclass, ObjectSubclassIsExt};
use glib::subclass::InitializingObject;
use std::cell::RefCell;

#[derive(Default)]
pub struct Foo {
    pub a: RefCell<i32>,
    pub b: RefCell<i32>,
}


#[repr(C)]
pub struct FooClass {
    pub parent_class: glib::gobject_ffi::GObjectClass,
    pub get_a:Option<unsafe extern "C" fn(*mut Foo) -> i32>,
    pub set_a:Option<unsafe extern "C" fn(*mut Foo, value: i32)>,
    pub get_b:Option<unsafe extern "C" fn(*mut Foo) -> i32>,
    pub set_b:Option<unsafe extern "C" fn(*mut Foo, value: i32)>,
}

unsafe impl ClassStruct for FooClass {
    type Type = Foo;
}

#[glib::object_subclass]
impl ObjectSubclass for Foo {
    const NAME: &'static str = "Foo";
    const ABSTRACT: bool = false;
    type Type = super::Foo;
    type Class = FooClass;

    fn instance_init(_obj: &InitializingObject<Self>) {
        unsafe {
            let r = _obj.as_ref().imp();
            *(r.a.borrow_mut()) = 1;
            *(r.b.borrow_mut()) = 2;
        }
    }

    fn class_init(klass: &mut Self::Class) {
        klass.get_a = Some(get_a);
        klass.set_a = Some(set_a);
        klass.get_b = Some(get_b);
        klass.set_b = Some(set_b);
    }
}

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

unsafe extern "C" fn get_a(this: *mut Foo) -> i32 { (*this).a() }
unsafe extern "C" fn set_a(this: *mut Foo, value: i32) { (*this).set_a(value) }
unsafe extern "C" fn get_b(this: *mut Foo) -> i32 { (*this).b() }
unsafe extern "C" fn set_b(this: *mut Foo, value: i32) { (*this).set_b(value) }
