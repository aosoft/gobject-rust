use glib::subclass::types::{ClassStruct, InstanceStructExt, ObjectSubclass};

#[repr(C)]
pub struct Foo {
    pub parent: glib::gobject_ffi::GObject,
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
    type Type = super::imp::Foo;
}

#[no_mangle]
pub unsafe extern "C" fn foo_get_a(this: *mut Foo) -> i32 {
    let instance = &*(this as *const <super::imp::Foo as ObjectSubclass>::Instance);
    instance.imp().a()
}

#[no_mangle]
pub unsafe extern "C" fn foo_set_a(this: *mut Foo, value: i32) {
    let instance = &*(this as *const <super::imp::Foo as ObjectSubclass>::Instance);
    instance.imp().set_a(value)
}

#[no_mangle]
pub unsafe extern "C" fn foo_get_b(this: *mut Foo) -> i32 {
    let instance = &*(this as *const <super::imp::Foo as ObjectSubclass>::Instance);
    instance.imp().b()
}

#[no_mangle]
pub unsafe extern "C" fn foo_set_b(this: *mut Foo, value: i32) {
    let instance = &*(this as *const <super::imp::Foo as ObjectSubclass>::Instance);
    instance.imp().set_b(value)
}
