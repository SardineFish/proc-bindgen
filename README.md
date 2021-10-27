# proc-bindgen

Procedrually create Rust bindings just like proc-macro.

This crate provide a simple way to generate custom binding code for Rust.

You generate whatever you like, e.g. a C++ template specialization.

### Example

```rust
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
```

To generate a header with complicated code:

```c++
void do_with_Foo(App* app, Foo obj);

template<>
void App::do_with<Foo>(Foo obj) {
    do_with_Foo(this, event);
}
```

With a custom generator
```rust
// my_gen

use proc_bindgen::{proc_code, BindgenBuilder, ProcBindgen};
use syn::{ItemStruct, __private::quote::format_ident};

pub struct MyProcBindgen;

impl MyProcBindgen {
    pub fn create() -> ProcBindgen {
        let mut builder = BindgenBuilder::new();

        builder.gen_struct("my_bindgen", gen_struct);

        builder.build()
    }
}

pub fn gen_struct(item: &ItemStruct) -> String {
    let name = item.ident.clone();
    let extern_fn_name = format_ident!("do_with_{}", name);

    proc_code!(

        void #extern_fn_name(App* app, #name obj);

        template<>
        void App::do_with<#name>(#name obj) {
            #extern_fn_name(this, event);
        }
    )
}

```

Use it in `build.rs`
```
use my_gen::MyProcBindgen;

fn main() {
    MyProcBindgen::create()
        .input("src/main.rs")
        .output("src/binding.hpp");

    println!("cargo:rerun-if-changed=src/main.rs");
}

```

Run `cargo +nightly build` to generate a binding file at `src/binding.hpp`