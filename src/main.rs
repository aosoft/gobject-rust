mod bar;
mod foo;

use crate::foo::FooExt;
use glib::ObjectExt;

fn main() {
    let foo = glib::object::Object::new::<foo::Foo>(&[]);
    println!("{}", foo.ref_count());

    println!("{}, {}", foo.a(), foo.b());
    foo.set_a(10);
    foo.set_b(20);
    println!("{}, {}", foo.a(), foo.b());

    let bar = glib::object::Object::new::<bar::Bar>(&[]);
    println!("{}", bar.ref_count());

    println!("{}, {}, {}, {}", bar.a(), bar.b(), bar.c(), bar.d());
    bar.set_a(10);
    bar.set_b(20);
    bar.set_c(30);
    bar.set_d(40);
    println!("{}, {}, {}, {}", bar.a(), bar.b(), bar.c(), bar.d());

    unsafe {
        let bar = bar::ffi::bar_new();
        println!(
            "{}, {}, {}, {}",
            foo::ffi::foo_get_a(bar as *mut _),
            foo::ffi::foo_get_b(bar as *mut _),
            bar::ffi::bar_get_c(bar),
            bar::ffi::bar_get_d(bar)
        );
        foo::ffi::foo_set_a(bar as *mut _, 10);
        foo::ffi::foo_set_b(bar as *mut _, 20);
        bar::ffi::bar_set_c(bar, 30);
        bar::ffi::bar_set_d(bar, 40);
        println!(
            "{}, {}, {}, {}",
            foo::ffi::foo_get_a(bar as *mut _),
            foo::ffi::foo_get_b(bar as *mut _),
            bar::ffi::bar_get_c(bar),
            bar::ffi::bar_get_d(bar)
        );
        glib::gobject_ffi::g_object_unref(bar as *mut _);
    }

    println!("end");
}
