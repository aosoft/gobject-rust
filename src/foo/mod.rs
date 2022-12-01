use glib::subclass::types::{IsSubclassable, ObjectSubclassExt, ObjectSubclassIsExt};
use glib::{IsA, ObjectExt, ObjectType};

pub(crate) mod ffi;
pub(crate) mod imp;

glib::wrapper! {
    pub struct Foo(ObjectSubclass<imp::Foo>);
}

unsafe impl<T: imp::FooImpl> IsSubclassable<T> for Foo {}

pub trait FooExt {
    fn a(&self) -> i32;
    fn set_a(&self, value: i32);
    fn b(&self) -> i32;
    fn set_b(&self, value: i32);
}

impl<O: IsA<Foo>> FooExt for O {
    fn a(&self) -> i32 {
        unsafe {
            let klass = self.as_ref().class();
            (klass.as_ref().get_a.unwrap())(
                self.as_ref().imp().instance().as_ptr() as *mut ffi::Foo
            )
        }
    }
    fn set_a(&self, value: i32) {
        unsafe {
            let klass = self.as_ref().class();
            (klass.as_ref().set_a.unwrap())(
                self.as_ref().imp().instance().as_ptr() as *mut ffi::Foo,
                value,
            )
        }
    }
    fn b(&self) -> i32 {
        self.as_ref().imp().b()
    }
    fn set_b(&self, value: i32) {
        self.as_ref().imp().set_b(value)
    }
}
