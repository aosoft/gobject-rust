mod foo;
mod bar;

use glib::ObjectExt;
use crate::foo::FooExt;

fn main() {
    let mut foo = glib::object::Object::new::<foo::Foo>(&[]);
    println!("{}", foo.ref_count());

    println!("{}, {}", foo.a(), foo.b());
    foo.set_a(10);
    foo.set_b(20);
    println!("{}, {}", foo.a(), foo.b());

    let mut bar = glib::object::Object::new::<bar::Bar>(&[]);
    println!("{}", bar.ref_count());

    println!("{}, {}, {}, {}", bar.a(), bar.b(), bar.c(), bar.d());
    bar.set_a(10);
    bar.set_b(20);
    bar.set_c(30);
    bar.set_d(40);
    println!("{}, {}, {}, {}", bar.a(), bar.b(), bar.c(), bar.d());
}
