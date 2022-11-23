use glib::subclass::object::ObjectImpl;
use glib::subclass::types::{IsSubclassable, ObjectSubclassIsExt};
use glib::translate::ToGlibPtr;
use glib::{IsA, ObjectExt};

pub(crate) mod imp;

glib::wrapper! {
    pub struct Foo(ObjectSubclass<imp::Foo>);
}

pub trait FooImpl: ObjectImpl + 'static {
    fn parent_get_a(&self, obj: &Foo) -> i32 {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut imp::FooClass;
            if let Some(ref f) = (*parent_class).get_a {
                f(obj.imp() as *const imp::Foo as *mut imp::Foo)
            } else {
                unimplemented!()
            }
        }
    }
    fn parent_set_a(&self, obj: &Foo, value: i32) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut imp::FooClass;
            if let Some(ref f) = (*parent_class).set_a {
                f(obj.imp() as *const imp::Foo as *mut imp::Foo, value)
            } else {
                unimplemented!()
            }
        }
    }
    fn parent_get_b(&self, obj: &Foo) -> i32 {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut imp::FooClass;
            if let Some(ref f) = (*parent_class).get_b {
                f(obj.imp() as *const imp::Foo as *mut imp::Foo)
            } else {
                unimplemented!()
            }
        }
    }
    fn parent_set_b(&self, obj: &Foo, value: i32) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *mut imp::FooClass;
            if let Some(ref f) = (*parent_class).set_b {
                f(obj.imp() as *const imp::Foo as *mut imp::Foo, value)
            } else {
                unimplemented!()
            }
        }
    }
}

unsafe impl<T: FooImpl> IsSubclassable<T> for Foo {}

pub trait FooExt {
    fn a(&self) -> i32;
    fn set_a(&self, value: i32);
    fn b(&self) -> i32;
    fn set_b(&self, value: i32);
}

/*
impl<O: IsA<Foo>> FooExt for O {
    fn a(&self) -> i32 { self.as_ref().imp().a() }
    fn set_a(&self, value: i32) { self.as_ref().imp().set_a(value) }
    fn b(&self) -> i32 { self.as_ref().imp().b() }
    fn set_b(&self, value: i32) { self.as_ref().imp().set_b(value) }
}
*/

impl<O: IsA<Foo>> FooExt for O {
    fn a(&self) -> i32 {
        unsafe {
            let klass = self.as_ref().class();
            (klass.as_ref().get_a.unwrap())(self.as_ref().imp() as *const imp::Foo as *mut imp::Foo)
        }
    }
    fn set_a(&self, value: i32) {
        unsafe {
            let klass = self.as_ref().class();
            (klass.as_ref().set_a.unwrap())(
                self.as_ref().imp() as *const imp::Foo as *mut imp::Foo,
                value,
            )
        }
    }
    fn b(&self) -> i32 {
        unsafe {
            let klass = self.as_ref().class();
            (klass.as_ref().get_b.unwrap())(self.as_ref().imp() as *const imp::Foo as *mut imp::Foo)
        }
    }
    fn set_b(&self, value: i32) {
        unsafe {
            let klass = self.as_ref().class();
            (klass.as_ref().set_b.unwrap())(
                self.as_ref().imp() as *const imp::Foo as *mut imp::Foo,
                value,
            )
        }
    }
}
