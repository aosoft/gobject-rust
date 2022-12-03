use glib::ObjectType;
use glib::subclass::types::{ClassStruct, InstanceStructExt, ObjectSubclass};
use glib::translate::{IntoGlib, ToGlibPtr};

/*
#[repr(C)]
pub struct Foo {
    pub parent: glib::gobject_ffi::GObject,
}
 */
pub type Foo = <super::imp::Foo as ObjectSubclass>::Instance;

#[repr(C)]
pub struct FooClass {
    pub parent_class: glib::gobject_ffi::GObjectClass,
    pub get_a:Option<unsafe extern "C" fn(*mut Foo) -> i32>,
    pub set_a:Option<unsafe extern "C" fn(*mut Foo, value: i32)>,
}

unsafe impl ClassStruct for FooClass {
    type Type = super::imp::Foo;
}

#[no_mangle]
pub unsafe extern "C" fn foo_new() -> *mut Foo {
    let obj = glib::object::Object::new::<super::Foo>(&[]);
    obj.as_object_ref().to_glib_full() as *mut Foo
}

#[no_mangle]
pub extern "C" fn foo_get_type() -> glib::ffi::GType {
    <super::Foo as glib::StaticType>::static_type().into_glib()
}

#[no_mangle]
pub unsafe extern "C" fn foo_get_a(this: *mut Foo) -> i32 {
    (*this).imp().a()
}

#[no_mangle]
pub unsafe extern "C" fn foo_set_a(this: *mut Foo, value: i32) {
    (*this).imp().set_a(value)
}

#[no_mangle]
pub unsafe extern "C" fn foo_get_b(this: *mut Foo) -> i32 {
    (*this).imp().b()
}

#[no_mangle]
pub unsafe extern "C" fn foo_set_b(this: *mut Foo, value: i32) {
    (*this).imp().set_b(value)
}
