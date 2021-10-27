use proc_bindgen_macro::*;

struct App {
    // ..
}

#[procbind(my_bindgen)]
struct Foo {
    // ..
}

extern "C" fn do_with_Foo(app: *mut App, foo: Foo) {
    // ..
}

fn main() {
    println!("Hello, world!");
}
