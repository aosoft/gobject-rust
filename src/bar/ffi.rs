use glib::subclass::types::ClassStruct;

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
