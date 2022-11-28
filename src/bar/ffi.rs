use glib::ObjectType;
use glib::subclass::types::{ClassStruct, InstanceStructExt, ObjectSubclass};
use glib::translate::{IntoGlib, ToGlibPtr};

#[repr(C)]
pub struct Bar {
    pub parent: crate::foo::ffi::Foo,
}

#[repr(C)]
pub struct BarClass {
    pub parent_class: crate::foo::ffi::FooClass,
}

unsafe impl ClassStruct for BarClass {
    type Type = super::imp::Bar;
}

#[no_mangle]
pub unsafe extern "C" fn bar_new() -> *mut Bar {
    let obj = glib::object::Object::new::<super::Bar>(&[]);
    obj.as_object_ref().to_glib_full() as *mut Bar
}

#[no_mangle]
pub extern "C" fn bar_get_type() -> glib::ffi::GType {
    <super::Bar as glib::StaticType>::static_type().into_glib()
}

#[no_mangle]
pub unsafe extern "C" fn bar_get_c(this: *mut Bar) -> i32 {
    let instance = &*(this as *const <super::imp::Bar as ObjectSubclass>::Instance);
    instance.imp().c()
}

#[no_mangle]
pub unsafe extern "C" fn bar_set_c(this: *mut Bar, value: i32) {
    let instance = &*(this as *const <super::imp::Bar as ObjectSubclass>::Instance);
    instance.imp().set_c(value)
}

#[no_mangle]
pub unsafe extern "C" fn bar_get_d(this: *mut Bar) -> i32 {
    let instance = &*(this as *const <super::imp::Bar as ObjectSubclass>::Instance);
    instance.imp().d()
}

#[no_mangle]
pub unsafe extern "C" fn bar_set_d(this: *mut Bar, value: i32) {
    let instance = &*(this as *const <super::imp::Bar as ObjectSubclass>::Instance);
    instance.imp().set_d(value)
}
